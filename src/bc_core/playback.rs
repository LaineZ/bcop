use std::io::Read;
use std::sync::atomic::{AtomicU16, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{
    fmt::{self, Display},
    ops::Neg,
};

use anyhow::{anyhow, Context, Result};
use minimp3::Decoder;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, ChannelCount, Sample, SampleFormat, SampleRate, Stream, StreamConfig};
use samplerate::{convert, ConverterType};

struct TimeTracker {
    started_at: Instant,
    paused_at: Option<Instant>,
    pause_time: Duration,
}

impl TimeTracker {
    fn new() -> Self {
        Self {
            started_at: Instant::now(),
            paused_at: None,
            pause_time: Duration::from_secs(0),
        }
    }

    fn reset(&mut self) {
        *self = TimeTracker::new();
    }

    fn pause(&mut self) {
        if self.paused_at.is_none() {
            self.paused_at = Some(Instant::now());
        }
    }

    fn play(&mut self) {
        if let Some(at) = self.paused_at.take() {
            self.pause_time += at.elapsed();
        }
    }

    fn seek_forward(&mut self, t: Duration) {
        self.started_at -= t;
    }

    fn seek_backward(&mut self, t: Duration) {
        self.started_at += t;
    }

    fn time(&self) -> Duration {
        if let Some(at) = self.paused_at {
            self.started_at.elapsed() - self.pause_time - at.elapsed()
        } else {
            self.started_at.elapsed() - self.pause_time
        }
    }
}

pub struct FormatTime(pub Duration);

impl Display for FormatTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_secs = self.0.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        write!(f, "{:02}:{:02}", mins, secs)
    }
}

fn time_to_samples(time: Duration, rate: SampleRate, channels: ChannelCount) -> usize {
    (time.as_secs_f64() * (rate.0 as f64) * (channels as f64)) as usize
}

#[derive(Debug)]
enum Command {
    GetTime,
    SwitchTrack(String),
    Play,
    Pause,
    Stop,
    SeekForward(Duration),
    SeekBackwards(Duration),
    AddVolume(u16),
}

struct Buffer {
    frames: Mutex<Vec<Vec<i16>>>,
    remaining_samples: AtomicUsize,
    submitted_samples: AtomicUsize,
    volume: AtomicU16,
}

struct PlayerThread {
    cmd_rx: Receiver<Command>,
    time_tx: Sender<Option<Duration>>,
    buffer: Arc<Buffer>,
    decoder: Option<Decoder<Box<dyn Read>>>,
    stream: Stream,
    tracker: TimeTracker,
    is_playing: bool,
    sample_rate: u32,
}

fn load_track(url: &str) -> Option<Decoder<Box<dyn Read>>> {
    let agent = ureq::builder()
        .timeout_connect(std::time::Duration::from_secs(10))
        .timeout_read(Duration::from_secs(5))
        .build();

    let mut tries = 0;
    while tries < 10 {
        let reader = agent.get(&url).call();

        match reader {
            Ok(r) => {
                log::info!("Started playback!");
                return Some(Decoder::new(Box::new(r.into_reader())));
            }
            Err(error) => {
                log::error!("Cannot start playback: {}", error.to_string());
                tries += 1;
                continue;
            }
        }
    }

    None
}

impl PlayerThread {
    fn new(cmd_rx: Receiver<Command>, time_tx: Sender<Option<Duration>>) -> Result<Self> {
        log::info!("Starting player thread");

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no output device available"))?;
        let device_name = device.name().unwrap_or("unknown".to_string());
        log::info!("Audio device: {}", device_name);

        let mut stream_config = StreamConfig {
            channels: 2,
            buffer_size: BufferSize::Default,
            sample_rate: SampleRate(44100),
        };

        let supported_configs = device
            .supported_output_configs()
            .context("Can't querying device configs")?;
        log::info!("Supported configurations:");
        let mut selected = None;
        let mut format = SampleFormat::I16;
        for (i, config) in supported_configs.enumerate() {
            let (min, max) = (config.min_sample_rate().0, config.max_sample_rate().0);
            let cur_format = config.sample_format();
            log::info!(
                " - {:2}. channels: {}; min samplerate: {}; max: {}; {:?}",
                i,
                config.channels(),
                min,
                max,
                cur_format,
            );

            if max >= 44100 && config.channels() == 2 {
                if selected.is_none() || cur_format == SampleFormat::I16 {
                    format = config.sample_format();
                    if cfg!(target_os = "windows") {
                        stream_config.sample_rate = SampleRate(max);
                    }
                    selected = Some(i);
                }
            }
        }

        if let Some(v) = selected {
            log::info!("Selected configuration {}", v);
        } else {
            log::info!("Device not supported");
        }

        let buffer = Arc::new(Buffer {
            frames: Mutex::new(Vec::new()),
            remaining_samples: AtomicUsize::new(0),
            submitted_samples: AtomicUsize::new(0),
            volume: AtomicU16::new(100),
        });

        fn data_fn<T: Sample>(output: &mut [T], buf: &Buffer) {
            let total = output.len();
            let mut filled = 0;

            //log::info!("Volume: {}", (buf.volume.load(Ordering::Relaxed) as f32) / 327.68);

            let skip = buf.remaining_samples.load(Ordering::SeqCst) == 0;
            let vol = buf.volume.load(Ordering::Relaxed) as f32 / 327.68;

            if !skip {
                let mut frames = match buf.frames.lock() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("{}", e);
                        return;
                    }
                };
                while !(frames.is_empty() || filled == total) {
                    let frame = &mut frames[0];
                    let len = frame.len().min(total - filled);
                    for i in 0..len {
                        frame[i] = (frame[i] as f32 * vol) as i16;
                        output[filled + i] = Sample::from(&frame[i])
                    }
                    filled += len;
                    if len == frame.len() {
                        frames.remove(0);
                    } else {
                        frame.drain(..len);
                    }
                }
            }

            while filled < total {
                output[filled] = Sample::from(&0.0);
                filled += 1;
            }

            buf.submitted_samples.fetch_add(total, Ordering::SeqCst);
            buf.remaining_samples.fetch_sub(filled, Ordering::SeqCst);
        };

        let error_fn = move |e| {
            log::error!("Stream error: {}", e);
        };

        let b = buffer.clone();
        let stream = match format {
            SampleFormat::I16 => {
                device.build_output_stream(&stream_config, move |v, _| data_fn::<i16>(v, &b), error_fn)?
            }
            SampleFormat::U16 => {
                device.build_output_stream(&stream_config, move |v, _| data_fn::<u16>(v, &b), error_fn)?
            }
            SampleFormat::F32 => {
                device.build_output_stream(&stream_config, move |v, _| data_fn::<f32>(v, &b), error_fn)?
            }
        };
        stream.play()?;

        Ok(Self {
            cmd_rx,
            time_tx,
            buffer,
            decoder: None,
            stream,
            tracker: TimeTracker::new(),
            is_playing: true,
            sample_rate: stream_config.sample_rate.0
        })
    }

    fn skip_samples(&mut self, mut num: usize) -> Result<()> {
        let skipping = num;
        self.buffer.remaining_samples.store(0, Ordering::SeqCst);

        let mutex = &self.buffer.frames;
        let mut frames = mutex.lock().map_err(|_| anyhow!("Can't lock mutex"))?;

        while !(frames.is_empty() || num == 0) {
            let frame = &mut frames[0];
            let count = num.min(frame.len());
            frame.drain(..count);
            if count == frame.len() {
                frames.remove(0);
            }
            num -= count;
        }

        drop(frames);

        while num > 0 {
            let mut frame = match self.next_frame()? {
                Some(v) => v,
                None => break,
            };
            let count = num.min(frame.data.len());
            frame.data.drain(..count);
            num -= count;
            if count < frame.data.len() {
                let mutex = &self.buffer.frames;
                let mut frames = mutex.lock().map_err(|_| anyhow!("Can't lock mutex"))?;
                self.buffer
                    .remaining_samples
                    .store(frame.data.len(), Ordering::SeqCst);
                frames.push(frame.data);
            }
        }

        self.buffer
            .submitted_samples
            .fetch_add(skipping, Ordering::SeqCst);

        Ok(())
    }

    fn next_frame(&mut self) -> Result<Option<minimp3::Frame>> {
        let decoder = match &mut self.decoder {
            Some(v) => v,
            None => return Ok(None),
        };
        match decoder.next_frame() {
            Ok(v) => Ok(Some(v)),
            Err(minimp3::Error::Eof) => {
                self.decoder = None;
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn reset(&mut self) {
        self.decoder = None;
        if let Ok(mut frames) = self.buffer.frames.lock() {
            frames.clear();
        }
        self.buffer.submitted_samples.store(0, Ordering::SeqCst);
        self.buffer.remaining_samples.store(0, Ordering::SeqCst);
        self.tracker.reset();
        self.is_playing = false;
    }

    fn run(mut self) -> Result<()> {
        let mut cur_url = None;

        loop {
            let timeout = Duration::from_millis(10);

            if self.decoder.is_none() {
                // track almost ended
                if self.buffer.remaining_samples.load(Ordering::SeqCst) == 0 {
                    // track ended
                    self.reset();
                }
            }

            let cmd = match self.cmd_rx.recv_timeout(timeout) {
                Ok(c) => c,
                Err(RecvTimeoutError::Timeout) => {
                    if let Some(frame) = self.next_frame()? {
                        let len = frame.data.len();
                        if let Ok(mut frames) = self.buffer.frames.lock() {
                            if self.sample_rate > 44100
                            {
                                let frame_resamp: Vec<f32> =
                                    frame.data.iter().map(|&v| v as f32).collect();
                                let resampled = convert(
                                    44100,
                                    self.sample_rate,
                                    2,
                                    ConverterType::Linear,
                                    frame_resamp.as_slice(),
                                )
                                .unwrap();
                                frames.push(resampled.iter().map(|&v| v as i16).collect());
                            }
                            else
                            {
                                frames.push(frame.data);
                            }
                        }
                        self.buffer
                            .remaining_samples
                            .fetch_add(len, Ordering::SeqCst);
                    }
                    continue;
                }
                _ => continue,
            };

            match cmd {
                Command::SwitchTrack(url) => {
                    log::info!("Loading track {}", &url);
                    self.reset();
                    self.decoder = load_track(&url);
                    self.is_playing = true;
                    cur_url = Some(url);
                }

                Command::Stop => {
                    cur_url = None;
                    self.reset();
                }

                Command::SeekForward(time) => {
                    if cur_url.is_none() {
                        continue;
                    }

                    self.tracker.seek_forward(time);
                    let samples = time_to_samples(time, SampleRate(self.sample_rate), 2);
                    self.skip_samples(samples)?;
                }

                Command::SeekBackwards(mut time) => {
                    if cur_url.is_none() {
                        continue;
                    }

                    time = std::cmp::min(time, self.tracker.time());
                    self.tracker.seek_backward(time);

                    self.decoder = load_track(cur_url.as_ref().unwrap());

                    let submitted = self.buffer.submitted_samples.load(Ordering::SeqCst);
                    let samples = time_to_samples(time, SampleRate(self.sample_rate), 2).min(submitted);
                    self.skip_samples(submitted - samples)?;
                    self.buffer
                        .submitted_samples
                        .store(submitted - samples, Ordering::SeqCst);
                }

                Command::Play => {
                    self.stream.play()?;
                    self.tracker.play();
                }

                Command::Pause => {
                    self.stream.pause()?;
                    self.tracker.pause();
                }

                Command::AddVolume(value) => {
                    self.buffer.volume.store(value, Ordering::Relaxed);
                }

                Command::GetTime => {
                    if self.is_playing {
                        self.time_tx.send(Some(self.tracker.time()))?;
                    } else {
                        self.time_tx.send(None)?;
                    }
                }
            }
        }
    }
}

pub struct Player {
    cmd_tx: Sender<Command>,
    time_rx: Receiver<Option<Duration>>,
    is_paused: bool,
    volume: u16,
}

impl Player {
    pub fn new() -> Player {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (time_tx, time_rx) = mpsc::channel();

        std::thread::Builder::new()
            .name("player".into())
            .spawn(move || {
                if let Err(e) = PlayerThread::new(cmd_rx, time_tx).and_then(|v| v.run()) {
                    log::error!("{}", e);
                }
            })
            .expect("Can't spawn player thread");

        Player {
            cmd_tx,
            time_rx,
            is_paused: false,
            volume: 100,
        }
    }

    fn send(&self, cmd: Command) {
        if self.cmd_tx.send(cmd).is_err() {
            log::error!("Player thread is dead");
        }
    }

    pub fn get_time(&self) -> Option<Duration> {
        let res = self
            .cmd_tx
            .send(Command::GetTime)
            .ok()
            .and_then(|_| self.time_rx.recv().ok());
        if let Some(r) = res {
            r
        } else {
            log::error!("Can't get time: player thread is dead");
            None
        }
    }

    pub fn is_playing(&self) -> bool {
        self.get_time().is_some()
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        if self.is_paused && !paused {
            self.play();
        } else if !self.is_paused && paused {
            self.pause();
        }
        self.is_paused = paused;
    }

    pub fn play(&mut self) {
        self.send(Command::Play);
    }

    pub fn pause(&mut self) {
        self.send(Command::Pause);
    }

    pub fn increase_volume(&mut self, value: u16) {
        self.volume += value;
        self.send(Command::AddVolume(self.volume));
    }

    pub fn decrease_volume(&mut self, value: u16) {
        if self.volume as i16 - value as i16 >= 0 {
            self.volume -= value;
            self.send(Command::AddVolume(self.volume));
        }
    }

    pub fn get_volume(&mut self) -> u16 {
        self.volume
    }

    pub fn stop(&mut self) {
        self.send(Command::Stop);
    }

    pub fn switch_track(&mut self, url: impl Into<String>) {
        self.send(Command::SwitchTrack(url.into()));
    }

    pub fn seek_forward(&mut self, time: Duration) {
        self.send(Command::SeekForward(time));
    }

    pub fn seek_backward(&mut self, time: Duration) {
        self.send(Command::SeekBackwards(time));
    }

    pub fn seek(&mut self, time: Duration) {
        let seek_secs = self.get_time().unwrap_or(Duration::from_secs(0)).as_secs() as i32
            - time.as_secs() as i32;

        if seek_secs > 0 {
            self.seek_backward(Duration::from_secs(seek_secs as u64));
        } else {
            self.seek_forward(Duration::from_secs(seek_secs.neg() as u64));
        }
    }
}

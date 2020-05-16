use std::fmt::{self, Display};
use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use minimp3::Decoder;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ChannelCount, SampleRate, Stream, StreamConfig};

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
        let micros = self.0.subsec_micros();
        write!(f, "{:02}:{:02}.{:06}", mins, secs, micros)
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
    AddVolume(f32),
}

struct Buffer {
    frames: Mutex<Vec<Vec<i16>>>,
    remaining_samples: AtomicUsize,
    submitted_samples: AtomicUsize,
}

struct PlayerThread {
    cmd_rx: Receiver<Command>,
    time_tx: Sender<Option<Duration>>,
    buffer: Arc<Buffer>,
    decoder: Option<Decoder<Box<dyn Read>>>,
    stream: Stream,
    tracker: TimeTracker,
    volume: f32,
}

fn load_track(url: &str) -> Decoder<Box<dyn Read>> {
    let reader = ureq::get(&url)
        .timeout_connect(5_000)
        .timeout_read(1_000)
        .call()
        .into_reader();
    Decoder::new(Box::new(reader))
}

impl PlayerThread {
    fn new(cmd_rx: Receiver<Command>, time_tx: Sender<Option<Duration>>) -> Result<Self> {
        log::info!("Starting player thread");

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no output device available"))?;

        let config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(44100),
        };

        let device_name = device.name().unwrap_or("Device name not found".to_string());
        log::info!(
            "Creating stream on {} with {} sample rate",
            device_name,
            config.sample_rate.0
        );

        let buffer = Arc::new(Buffer {
            frames: Mutex::new(Vec::new()),
            remaining_samples: AtomicUsize::new(0),
            submitted_samples: AtomicUsize::new(0),
        });

        let buf = buffer.clone();
        let data_fn = move |output: &mut [i16], _: &_| {
            let total = output.len();
            let mut filled = 0;

            let skip = buf.remaining_samples.load(Ordering::SeqCst) == 0;

            if !skip {
                let mut frames = buf.frames.lock().unwrap();
                while !(frames.is_empty() || filled == total) {
                    let frame = &mut frames[0];
                    let len = frame.len().min(total - filled);
                    output[filled..][..len].copy_from_slice(&frame[..len]);
                    filled += len;
                    if len == frame.len() {
                        frames.remove(0);
                    } else {
                        frame.drain(..len);
                    }
                }
            }

            while filled < total {
                output[filled] = 0;
                filled += 1;
            }

            buf.submitted_samples.fetch_add(total, Ordering::SeqCst);
            buf.remaining_samples.fetch_sub(filled, Ordering::SeqCst);
        };

        let error_fn = move |e| {
            log::error!("Stream error: {}", e);
        };

        let stream = device.build_output_stream(&config, data_fn, error_fn)?;
        stream.play()?;

        Ok(Self {
            cmd_rx,
            time_tx,
            buffer,
            decoder: None,
            stream,
            tracker: TimeTracker::new(),
            volume: 1.0,
        })
    }

    fn skip_samples(&mut self, mut num: usize) -> Result<()> {
        let skipping = num;
        self.buffer.remaining_samples.store(0, Ordering::SeqCst);

        let mut frames = self.buffer.frames.lock().unwrap();

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
                let mut frames = self.buffer.frames.lock().unwrap();
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
                self.tracker.reset();
                self.tracker.pause();
                Ok(None)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn reset(&mut self) {
        self.decoder = None;
        self.buffer.frames.lock().unwrap().clear();
        self.buffer.submitted_samples.store(0, Ordering::SeqCst);
        self.buffer.remaining_samples.store(0, Ordering::SeqCst);
        self.tracker.reset();
    }

    fn run(mut self) -> Result<()> {
        let mut cur_url = None;

        loop {
            let timeout = Duration::from_millis(10);

            let cmd = match self.cmd_rx.recv_timeout(timeout) {
                Ok(c) => c,
                Err(RecvTimeoutError::Timeout) => {
                    if let Some(frame) = self.next_frame()? {
                        let len = frame.data.len();
                        self.buffer.frames.lock().unwrap().push(frame.data);
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
                    self.decoder = Some(load_track(&url));
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
                    let samples = time_to_samples(time, SampleRate(44100), 2);
                    self.skip_samples(samples)?;
                }

                Command::SeekBackwards(mut time) => {
                    if cur_url.is_none() {
                        continue;
                    }

                    time = std::cmp::min(time, self.tracker.time());
                    self.tracker.seek_backward(time);

                    self.decoder = Some(load_track(cur_url.as_ref().unwrap()));

                    let submitted = self.buffer.submitted_samples.load(Ordering::SeqCst);
                    let samples = time_to_samples(time, SampleRate(44100), 2).min(submitted);
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
                    self.volume += value;
                }

                Command::GetTime => {
                    if self.decoder.is_some() {
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
    volume: f32,
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
            .unwrap();

        Player {
            cmd_tx,
            time_rx,
            is_paused: false,
            volume: 1.0,
        }
    }

    pub fn get_time(&self) -> Option<Duration> {
        self.cmd_tx.send(Command::GetTime).unwrap();
        self.time_rx.recv().unwrap()
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
        self.cmd_tx.send(Command::Play).unwrap();
    }

    pub fn pause(&mut self) {
        self.cmd_tx.send(Command::Pause).unwrap();
    }

    pub fn add_volume(&mut self, value: f32) {
        self.volume += value;
        self.cmd_tx.send(Command::AddVolume(value)).unwrap();
    }

    pub fn get_volume(&mut self) -> f32 {
        self.volume
    }

    pub fn stop(&mut self) {
        self.cmd_tx.send(Command::Stop).unwrap();
    }

    pub fn switch_track(&mut self, url: impl Into<String>) {
        self.cmd_tx.send(Command::SwitchTrack(url.into())).unwrap();
    }

    pub fn seek_forward(&mut self, time: Duration) {
        self.cmd_tx.send(Command::SeekForward(time)).unwrap();
    }

    pub fn seek_backward(&mut self, time: Duration) {
        self.cmd_tx.send(Command::SeekBackwards(time)).unwrap();
    }
}

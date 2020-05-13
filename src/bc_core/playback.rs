use std::fmt::{self, Display};
use std::io::Read;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use minimp3::Decoder;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ChannelCount, SampleRate, Stream, StreamConfig, StreamError};

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
    GetData(usize),
    StreamError(StreamError),
    AddVolume(f32),
}

struct PlayerThread {
    cmd_rx: Receiver<Command>,
    time_tx: Sender<Option<Duration>>,
    data_tx: Sender<Vec<i16>>,
    decoder: Option<Decoder<Box<dyn Read>>>,
    buffer: Vec<i16>,
    stream: Stream,
    samples_submitted: usize,
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
    fn new(
        cmd_tx: Sender<Command>,
        cmd_rx: Receiver<Command>,
        time_tx: Sender<Option<Duration>>,
    ) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no output device available"))?;

        let config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(44100),
        };

        let cmd_tx1 = cmd_tx.clone();
        let (data_tx, data_rx) = mpsc::channel::<Vec<i16>>();
        let data_fn = move |output: &mut [i16], _: &_| {
            let _ = cmd_tx1.send(Command::GetData(output.len()));
            if let Ok(data) = data_rx.recv_timeout(Duration::from_secs(1)) {
                if data.len() == output.len() {
                    output.copy_from_slice(&data);
                }
            }
        };

        let error_fn = move |e| {
            let _ = cmd_tx.send(Command::StreamError(e));
        };

        let stream = device.build_output_stream(&config, data_fn, error_fn)?;
        stream.play()?;

        Ok(Self {
            cmd_rx,
            time_tx,
            data_tx,
            decoder: None,
            buffer: Vec::new(),
            stream,
            samples_submitted: 0,
            tracker: TimeTracker::new(),
            volume: 1.0
        })
    }

    fn skip_samples(&mut self, mut num: usize) -> Result<()> {
        let count = num.min(self.buffer.len());
        self.buffer.drain(..count);
        num -= count;

        while num > 0 {
            let frame = match self.next_frame()? {
                Some(v) => v,
                None => break,
            };
            let count = num.min(frame.data.len());
            num -= count;
            if count < frame.data.len() {
                self.buffer.extend_from_slice(&frame.data[count..]);
            }
        }

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

    fn run(mut self) -> Result<()> {
        let mut cur_url = None;

        loop {
            let cmd = self.cmd_rx.recv()?;

            match cmd {
                Command::SwitchTrack(url) => {
                    self.decoder = Some(load_track(&url));
                    cur_url = Some(url);
                    self.buffer.clear();
                    self.samples_submitted = 0;
                    self.tracker.reset();
                }

                Command::Stop => {
                    cur_url = None;
                    self.decoder = None;
                    self.buffer.clear();
                    self.samples_submitted = 0;
                    self.tracker.reset();
                }

                Command::GetData(len) => {
                    while self.buffer.len() < len {
                        let frame = match self.next_frame()? {
                            Some(v) => v,
                            None => {
                                let mut rem = len - self.buffer.len();
                                while rem > 0 {
                                    self.buffer.push(0);
                                    rem -= 1;
                                }
                                break;
                            }
                        };
                        self.buffer.extend_from_slice(&frame.data);
                    }

                    let mut data: Vec<i16> = Vec::new();

                    for sample in self.buffer.drain(..len) {
                        data.push((sample as f32 * self.volume) as i16);
                    }
                    self.samples_submitted += len;
                    self.data_tx.send(data)?;
                }

                Command::SeekForward(time) => {
                    self.tracker.seek_forward(time);
                    let samples = time_to_samples(time, SampleRate(44100), 2);
                    self.skip_samples(samples)?;
                    self.samples_submitted += samples;
                }

                Command::SeekBackwards(mut time) => {
                    time = std::cmp::min(time, self.tracker.time());
                    self.tracker.seek_backward(time);

                    self.decoder = Some(load_track(cur_url.as_ref().unwrap()));

                    let samples =
                        time_to_samples(time, SampleRate(44100), 2).min(self.samples_submitted);

                    self.skip_samples(self.samples_submitted - samples)?;
                    self.samples_submitted -= samples;
                }

                Command::Play => {
                    self.stream.play()?;
                    self.tracker.play();
                }

                Command::Pause => {
                    self.stream.pause()?;
                    self.tracker.pause();
                }

                Command::StreamError(e) => {
                    // TODO: Make stream error
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
    volume: f32
}

impl Player {
    pub fn new() -> Player {
        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (time_tx, time_rx) = mpsc::channel();

        let tx = cmd_tx.clone();
        std::thread::spawn(move || {
            if let Err(e) = PlayerThread::new(tx, cmd_rx, time_tx).and_then(|v| v.run()) {
            }
        });

        Player {
            cmd_tx,
            time_rx,
            is_paused: false,
            volume: 1.0
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
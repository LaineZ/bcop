use std::fmt::{self, Display};
use std::io::Read;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use minimp3::Decoder;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ChannelCount, Device, Host, SampleRate, Stream, StreamConfig, StreamError};

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
        self.started_at.elapsed() - self.pause_time
    }
}

struct FormatTime(Duration);

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
pub enum Command {
    SwitchTrack(String),
    Play,
    Pause,
    SeekForward(Duration),
    SeekBackwards(Duration),
    GetData(usize),
    StreamError(StreamError),
}

struct PlayerThread {
    cmd_tx: Sender<Command>,
    cmd_rx: Receiver<Command>,
    data_tx: Option<Sender<Vec<i16>>>,
    buffer: Vec<i16>,
    device: Device,
    stream: Option<Stream>,
    samples_submitted: usize,
    tracker: TimeTracker,
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
    fn new(tx: Sender<Command>, rx: Receiver<Command>) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no output device available"))?;

        Ok(Self {
            cmd_tx: tx.clone(),
            cmd_rx: rx,
            data_tx: None,
            buffer: Vec::new(),
            device,
            stream: None,
            samples_submitted: 0,
            tracker: TimeTracker::new(),
        })
    }

    fn recreate_stream(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.pause();
        }

        let config = StreamConfig {
            channels: 2,
            sample_rate: SampleRate(44100),
        };

        let cmd_tx = self.cmd_tx.clone();
        let (data_tx, data_rx) = mpsc::channel::<Vec<i16>>();
        let data_fn = move |output: &mut [i16], _: &_| {
            let _ = cmd_tx.send(Command::GetData(output.len()));
            if let Ok(data) = data_rx.recv() {
                output.copy_from_slice(&data);
            }
        };

        let cmd_tx = self.cmd_tx.clone();
        let error_fn = move |e| {
            log::error!("stream error: {}", &e);
            let _ = cmd_tx.send(Command::StreamError(e));
        };

        let stream = self
            .device
            .build_output_stream(&config, data_fn, error_fn)?;
        stream.play()?;

        self.stream = Some(stream);
        self.data_tx = Some(data_tx);

        Ok(())
    }

    fn skip_samples(&mut self, decoder: &mut Decoder<impl Read>, mut num: usize) -> Result<()> {
        let count = num.min(self.buffer.len());
        self.buffer.drain(..count);
        num -= count;

        while num > 0 {
            let frame = decoder.next_frame()?;
            let count = num.min(frame.data.len());
            num -= count;
            if count < frame.data.len() {
                self.buffer.extend_from_slice(&frame.data[count..]);
            }
        }

        Ok(())
    }

    fn run(mut self) -> Result<()> {
        let mut decoder = None;
        let mut cur_url = None;

        loop {
            let cmd = self.cmd_rx.recv()?;

            match (cmd, &mut decoder, &mut self.data_tx, &mut self.stream) {
                (Command::SwitchTrack(url), ..) => {
                    decoder = Some(load_track(&url));
                    cur_url = Some(url);
                    self.buffer.clear();
                    self.recreate_stream()?;
                    self.samples_submitted = 0;
                }

                (Command::GetData(len), Some(decoder), Some(data_tx), ..) => {
                    while self.buffer.len() < len {
                        let frame = decoder.next_frame()?;
                        self.buffer.extend_from_slice(&frame.data);
                    }

                    let data = self.buffer.drain(..len).collect();
                    self.samples_submitted += len;
                    data_tx.send(data)?;
                }

                (Command::SeekForward(time), Some(decoder), ..) => {
                    self.tracker.seek_forward(time);
                    let samples = time_to_samples(time, SampleRate(44100), 2);
                    self.skip_samples(decoder, samples)?;
                    self.samples_submitted += samples;
                }

                (Command::SeekBackwards(time), Some(_), ..) => {
                    self.tracker.seek_backward(time);

                    decoder = Some(load_track(cur_url.as_ref().unwrap()));

                    let samples =
                        time_to_samples(time, SampleRate(44100), 2).min(self.samples_submitted);
                        
                    self.skip_samples(decoder.as_mut().unwrap(), self.samples_submitted - samples)?;
                    self.samples_submitted -= samples;
                }

                (Command::Play, .., Some(stream)) => {
                    stream.play()?;
                    self.tracker.play();
                }

                (Command::Pause, .., Some(stream)) => {
                    stream.pause()?;
                    self.tracker.pause();
                }

                _ => (),
            }
        }
    }
}
use std::{env, time::Duration};

use anyhow::bail;
use bass_rs::{
    prelude::{PlaybackState, StreamChannel},
    Bass,
};

use crate::players::Player;

pub struct BassPlayer {
    stream_channel: Option<StreamChannel>,
    _bass: Bass,
    sample_data: Vec<f32>,
    volume: f32,
}

impl BassPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let mut exe = env::current_exe().unwrap_or_default();

        exe.pop();

        if !exe.join("bass.dll").exists() {
            bail!("Bass library not found!")
        }

        let bass = Bass::init_default();

        match bass {
            Ok(b) => Ok(Self {
                stream_channel: None,
                sample_data: Vec::with_capacity(4096),
                _bass: b,
                volume: 1.0,
            }),
            Err(err) => bail!("Bass initialization error: {}", err),
        }
    }

    fn setup_stream_volume(&mut self) {
        if let Some(stream) = &self.stream_channel {
            stream.set_volume(self.volume).unwrap_or_else(|op| {
                log::error!("Unable to change volume due to error: {}", op);
            });
        }
    }
}

impl Player for BassPlayer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn get_time(&self) -> Option<std::time::Duration> {
        if let Some(stream) = &self.stream_channel {
            return Some(Duration::from_millis(
                stream.get_position().unwrap_or_default() as u64,
            ));
        }
        None
    }

    fn is_playing(&self) -> bool {
        if let Some(stream) = &self.stream_channel {
            //log::info!("{:?}", stream.get_playback_state());
            return stream
                .get_playback_state()
                .unwrap_or(PlaybackState::Stopped)
                == PlaybackState::Playing;
        }
        false
    }

    fn is_paused(&self) -> bool {
        if let Some(stream) = &self.stream_channel {
            return stream
                .get_playback_state()
                .unwrap_or(PlaybackState::Stopped)
                == PlaybackState::Paused;
        }
        true
    }

    fn set_paused(&mut self, paused: bool) {
        if let Some(stream) = &self.stream_channel {
            log::info!("{:?}", stream.get_playback_state());
            if paused {
                stream.pause().unwrap_or_else(|op| {
                    log::error!("Unable to switch state due to error: {}", op);
                });
            } else {
                stream.play(false).unwrap_or_else(|op| {
                    log::error!("Unable to switch state due to error: {}", op);
                });
            }
        }
    }

    fn set_volume(&mut self, value: u16) {
        self.volume = value as f32 / 100.0;
        self.setup_stream_volume();
    }

    fn get_volume(&mut self) -> u16 {
        if let Some(stream) = &self.stream_channel {
            (stream.get_volume().unwrap_or_default() * 100.0) as u16
        } else {
            0
        }
    }

    fn stop(&mut self) {
        if let Some(stream) = &self.stream_channel {
            stream.stop().unwrap_or_else(|op| {
                log::error!("I CANT STOP THAT: {}", op);
            });
        }
    }

    fn switch_track(&mut self, url: String) -> anyhow::Result<()> {
        if let Some(stream) = &self.stream_channel {
            stream
                .stop()
                .map_err(|e| anyhow::anyhow!("Failed to stop stream: {}", e))?;
            drop(stream);
            self.stream_channel = None;
        }
        let http = url.replace("https://", "http://");
        match StreamChannel::load_from_url(http, 0) {
            Ok(stream) => {
                stream
                    .play(true)
                    .map_err(|e| anyhow::anyhow!("Failed to start stream: {}", e))?;
                self.stream_channel = Some(stream);
                self.setup_stream_volume();
            }
            Err(err) => bail!("Unable to load stream: {}", err),
        }
        Ok(())
    }

    fn seek(&mut self, time: std::time::Duration) {
        if let Some(stream) = &self.stream_channel {
            stream
                .set_position(time.as_millis() as f64)
                .unwrap_or_else(|op| {
                    log::error!("Unable to seek: {}", op);
                });
        }
    }

    fn get_samples(&mut self) -> &[f32] {
        if self.is_playing() {
            if let Some(stream) = &self.stream_channel {
                match stream
                    .channel
                    .get_data(bass_rs::prelude::DataType::FFT4096, 4096)
                {
                    Ok(v) => {
                        self.sample_data = v;
                    }
                    Err(value) => {
                        log::warn!("Unable to get info: {}", value);
                    }
                }
            }
        } else {
            self.sample_data.clear();
            self.sample_data.fill(0.0);
        }

        &self.sample_data
    }
}

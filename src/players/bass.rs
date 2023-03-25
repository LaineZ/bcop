use std::{time::Duration, env};

use anyhow::bail;
use bass_rs::{
    prelude::{PlaybackState, StreamChannel},
    Bass,
};

use crate::players::Player;

pub struct BassPlayer {
    stream_channel: Option<StreamChannel>,
    _bass: Bass,
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
            Ok(b) => {
                Ok(Self {
                    stream_channel: None,
                    _bass: b,
                })
            },
            Err(err) => bail!("Bass initialization error: {:?}", err),
        }
    }
}

impl Player for BassPlayer {
    fn restart_on_fault(&self) -> bool {
        false
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
            log::info!("{:?}", stream.get_playback_state());
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
                    log::error!("Unable to switch state due to error: {:?}", op);
                });
            } else {
                stream.play(false).unwrap_or_else(|op| {
                    log::error!("Unable to switch state due to error: {:?}", op);
                });
            }
        }
    }

    fn set_volume(&mut self, value: u16) {
        if let Some(stream) = &self.stream_channel {
            stream
                .set_volume(value as f32 / 100.0)
                .unwrap_or_else(|op| {
                    log::error!("Unable to change volume due to error: {:?}", op);
                });
        }
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
                log::error!("I CANT STOP THAT: {:?}", op);
            });
        }
    }

    fn switch_track(&mut self, url: String) {
        if let Some(stream) = &self.stream_channel {
            stream.stop().unwrap_or_else(|op| {
                log::error!("Failed to start stop stream: {:?}", op);
            });
            self.stream_channel = None;
        }
        let http = url.replace("https://", "http://");
        match StreamChannel::load_from_url(http.clone(), 0) {
            Ok(stream) => {
                stream.play(true).unwrap_or_else(|op| {
                    log::error!("Failed to start start stream: {:?}", op);
                });
                self.stream_channel = Some(stream);
            }
            Err(err) => log::error!("Unable to load stream: {:?} {}", err, http),
        }
    }

    fn seek(&mut self, time: std::time::Duration) {
        if let Some(stream) = &self.stream_channel {
            stream
                .set_position(time.as_millis() as f64)
                .unwrap_or_else(|op| {
                    log::error!("Unable to seek: {:?}", op);
                });
        }
    }
}

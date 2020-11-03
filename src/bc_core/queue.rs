use core::fmt;
use std::{fmt::Display, time::Duration};

use super::album_parsing;

#[derive(Clone)]
pub struct QueuedTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub audio_url: String,
    pub album_url: String,
    pub duration: Duration,
}

impl Default for QueuedTrack {
    fn default() -> Self {
        QueuedTrack {
            title: String::new(),
            artist: String::new(),
            audio_url: String::new(),
            album: String::new(),
            album_url: String::new(),
            duration: Duration::from_secs(0),
        }
    }
}

impl Display for QueuedTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.artist, self.title)
    }
}

pub struct Queue {
    pub queue: Vec<QueuedTrack>,
    pub shuffle: bool,
    pub queue_pos: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            shuffle: false,
            queue_pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<QueuedTrack> {
        if self.queue_pos < self.queue.len() {
            self.queue_pos += 1;
            Some(self.queue[self.queue_pos].clone())
        } else {
            None
        }
    }

    pub fn set(&mut self, idx: usize) -> Option<QueuedTrack> {
        if idx < self.queue.len() {
            self.queue_pos = idx;

            Some(self.queue[self.queue_pos].clone())
        } else {
            None
        }
    }

    pub fn prev(&mut self) -> Option<QueuedTrack> {
        if self.queue_pos > 0 {
            self.queue_pos -= 1;

            Some(self.queue[self.queue_pos].clone())
        } else {
            None
        }
    }

    pub fn get_current_track(&mut self) -> Option<QueuedTrack> {
        if self.queue.len() > 0 {
            return Some(self.queue[self.queue_pos].clone());
        }
        None
    }

    pub fn add_album_in_queue(&mut self, artist: String, url: &str) -> Result<(), ()> {
        let is_album = album_parsing::get_album(url);

        match is_album {
            Some(album) => {
                for album_track in album.trackinfo.unwrap() {
                    match album_track.file.clone() {
                        Some(album_url) => {
                            let pushed_track = QueuedTrack {
                                album: album
                                    .current
                                    .clone()
                                    .title
                                    .unwrap_or("Unknown album".to_string()),
                                artist: artist.clone(),
                                title: album_track
                                    .title
                                    .unwrap_or("Unknown track title".to_string()),
                                // TODO: switch to normal error-handling and not this garbage that panic...
                                audio_url: album_track.file.unwrap().mp3128,
                                album_url: url.to_string(),
                                duration: Duration::from_secs_f64(
                                    album_track.duration.unwrap_or(0.0),
                                ),
                            };
                            self.queue.push(pushed_track.clone());
                        }
                        None => {}
                    }
                }
            }
            _ => return Err(()),
        }
        Ok(())
    }
}

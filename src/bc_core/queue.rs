use core::fmt;
use std::{any, fmt::Display, time::Duration};

use crate::model::album::{Album, Trackinfo};

use super::album_parsing;
use anyhow::anyhow;

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


impl QueuedTrack {
    fn new(track: &Trackinfo, album: Album) -> Option<Self> {
        if !track.unreleased_track.unwrap_or(false) || track.file.is_some()
        {
            log::info!("inserted!");
            return Some(QueuedTrack {
                title: track.title.as_deref().unwrap_or("Unknown title").to_string(),
                artist: album.current.artist.unwrap_or(String::from("Unknown artist")),
                audio_url: track.file.clone().unwrap().mp3128,
                album: album.current.title.unwrap_or(String::from("Unknown album name")),
                album_url: album.url.unwrap_or(String::from("https://bandcamp.com")),
                duration: Duration::from_secs(0),
            })
        }
        None
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
        if self.queue_pos + 1 < self.queue.len() - 1 {
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

    pub fn add_album_in_queue(&mut self, url: &str) -> Result<(), anyhow::Error> {
        let album = album_parsing::get_album(url).ok_or_else(|| anyhow!("Cannot get album data"))?;

        let trks = album.clone().trackinfo.ok_or_else(|| anyhow!("Unable to get tracklist"))?;
        if !trks.is_empty() {
            for track in trks.iter() {
                self.queue.push(QueuedTrack::new(track, album.clone()).ok_or_else(|| anyhow!("Cannot get track URL!"))?);
            }
            Ok(())
        }
        else {
            Err(anyhow!("Cannot get track URL!"))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::bc_core::queue::Queue;
    #[test]
    fn get_album_tracks_generic() {
        let mut queue = Queue::new();
        assert_eq!(true, queue.add_album_in_queue("https://masterbootrecord.bandcamp.com/album/interrupt-request").is_ok());
    }

    #[test]
    fn get_album_tracks_count() -> Result<(), anyhow::Error> {
        let mut queue = Queue::new();
        queue.add_album_in_queue("https://masterbootrecord.bandcamp.com/album/interrupt-request")?;
        assert_eq!(false, queue.queue.is_empty());

        Ok(())
    }

    #[test]
    fn get_album_tracks_empty() -> Result<(), anyhow::Error> {
        let mut queue = Queue::new();
        assert_eq!(true, queue.add_album_in_queue("https://sierpienrecords.bandcamp.com/album/etazhi-distro").is_err());
        assert_eq!(true, queue.queue.is_empty());

        Ok(())
    }
}

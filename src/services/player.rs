use anyhow::bail;
use regex::Regex;

use crate::players::{self, bass::BassPlayer, AudioSystem};


#[derive(Clone)]
pub struct QueuedTrack {
    pub title: String,
    pub album: String,
    pub album_url: String,
    pub mp3_url: Option<String>,
}

pub struct Player {
    pub player: Box<dyn players::Player>,
    pub queue: Vec<QueuedTrack>,
    queue_position: usize,
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}


fn fix_json(data: &str) -> String {
    // fix url field
    let regex = Regex::new("(?P<root>url: \".+)\" \\+ \"(?P<album>.+\",)").unwrap();
    let data = regex.replace_all(data, "$root$album");

    // add quotes to fields
    let regex = Regex::new("    (?P<property>[a-zA-Z_]+):").unwrap();
    let data = regex.replace_all(&data, "\"$property\":");

    // remove comments
    let regex = Regex::new("// .*").unwrap();
    let data = regex.replace_all(&data, "");

    data.into()
}

fn parse_album(html_code: String) -> Option<String> {
    let start = "data-tralbum=\"{";
    let stop = "}\"";

    let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
    let album_data = &album_data[..=album_data.find(stop)?];
    let album_data_json = fix_json(&album_data.replace("&quot;", "\""));
    Some(album_data_json)
}

impl Player {
    pub fn new(audio_system: AudioSystem, device_index: usize) -> Self {
        let bass = BassPlayer::new(device_index).expect("Unable to initialize bass library");

        match audio_system {
            AudioSystem::Bass => {
                Self {
                    player: Box::new(bass),
                    queue_position: 0,
                    queue: Vec::new(),
                }
            }
        }
    }

    pub fn get_current_track(&self) -> QueuedTrack {
        self.queue[self.queue_position].clone()
    }

    pub fn prev(&mut self) -> anyhow::Result<()> {
        if self.queue_position > 0 {
            self.queue_position -= 1;
            self.load_track()?;
        } else {
            bail!("Queue is already at beginning!")
        }
        Ok(())
    }

    pub fn next(&mut self) -> anyhow::Result<()> {
        if self.queue_position < self.queue.len() - 1 {
            self.load_track()?;
        } else {
            bail!("Queue is already at end!")
        }

        Ok(())
    }

    pub fn load_track(&mut self) -> anyhow::Result<()> {
        if let Some(url) = self.get_current_track().mp3_url {
            self.player.switch_track(url)?;
        } else {
            // TODO: revoke track URL
        }
        Ok(())
    }

    pub async fn add_to_queue(&self, url: &str) -> anyhow::Result<()> {
        let body = reqwest::get(url)
        .await?
        .text()
        .await?;

        let al_data = parse_album(body).unwrap();

        // TODO: add tracks to queue

        log::debug!("{}", fix_json(&al_data));

        Ok(())
    }
}
use std::sync::mpsc;

use anyhow::bail;
use raw_window_handle::Win32WindowHandle;
use regex::Regex;
use sciter::types::HWINDOW;
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};

use crate::players::{self, bass::BassPlayer, AudioSystem};

#[derive(Clone, Default)]
pub struct QueuedTrack {
    pub title: String,
    pub album: String,
    pub album_url: String,
    pub artist: String,
    pub mp3_url: Option<String>,
    pub img_url: String,
}

pub struct Player {
    rx: mpsc::Receiver<MediaControlEvent>,
    tx: mpsc::SyncSender<MediaControlEvent>,
    pub player: Box<dyn players::Player>,
    pub queue: Vec<QueuedTrack>,
    queue_position: usize,
    controls: Option<MediaControls>,
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
        let (tx, rx): (
            mpsc::SyncSender<MediaControlEvent>,
            mpsc::Receiver<MediaControlEvent>,
        ) = mpsc::sync_channel(32);
        match audio_system {
            AudioSystem::Bass => Self {
                rx,
                tx,
                controls: None,
                player: Box::new(bass),
                queue_position: 0,
                queue: Vec::new(),
            },
        }
    }

    pub fn setup_mediabuttons(&mut self, hwnd: Option<HWINDOW>) {
        #[cfg(target_os = "windows")]
        let hwnd = {
            use raw_window_handle::RawWindowHandle;

            let mut h = Win32WindowHandle::empty();
            h.hwnd = hwnd.unwrap() as *mut _;

            let handle = match RawWindowHandle::Win32(h) {
                RawWindowHandle::Win32(h) => h,
                _ => unreachable!(),
            };
            Some(handle.hwnd)
        };

        let config = PlatformConfig {
            dbus_name: "bc_rs",
            display_name: "BandcampOnlinePlayer",
            hwnd,
        };

        self.controls = Some(MediaControls::new(config).unwrap());

        let tx = self.tx.clone();
        self.controls
            .as_mut()
            .unwrap()
            .attach(move |e| {
                tx.send(e).unwrap();
            })
            .unwrap();

        self.controls
            .as_mut()
            .unwrap()
            .set_playback(MediaPlayback::Playing { progress: None })
            .unwrap();
    }

    pub fn update_metadata(&mut self) -> anyhow::Result<()> {
        let ct = self.get_current_track();

        self.controls
            .as_mut()
            .unwrap()
            .set_metadata(MediaMetadata {
                title: Some(&ct.title),
                album: Some(&ct.album),
                artist: Some(&ct.artist),
                duration: self.player.get_time(),
                cover_url: Some(&ct.img_url),
            })
            .map_err(|e| anyhow::anyhow!("Failed to set device: {:?}", e))
    }

    pub fn process_mediabutton_events(&mut self) {
        let is_paused = self.player.is_paused();

        if let Ok(event) = self.rx.try_recv() {
            match event {
                MediaControlEvent::Toggle => self.player.set_paused(!is_paused),
                MediaControlEvent::Play => self.player.set_paused(false),
                MediaControlEvent::Pause => self.player.set_paused(true),
                MediaControlEvent::Stop => self.player.stop(),
                MediaControlEvent::SeekBy(_, duration) => self.player.seek(duration),
                _ => (),
            }
        }
    }

    pub fn set_paused(&mut self, state: bool) {
        self.player.set_paused(state);
        if state {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Paused {
                    progress: Some(souvlaki::MediaPosition(
                        self.player.get_time().unwrap_or_default(),
                    )),
                })
                .ok();
        } else {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(
                        self.player.get_time().unwrap_or_default(),
                    )),
                })
                .ok();
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
            self.update_metadata();
        } else {
            // TODO: revoke track URL
        }
        Ok(())
    }

    pub async fn add_to_queue(&self, url: &str) -> anyhow::Result<()> {
        let body = reqwest::get(url).await?.text().await?;

        let al_data = parse_album(body).unwrap();

        // TODO: add tracks to queue

        log::debug!("{}", fix_json(&al_data));

        Ok(())
    }
}

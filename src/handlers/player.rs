use std::{
    sync::{mpsc, Arc},
    time::Duration
};

use anyhow::bail;
use raw_window_handle::Win32WindowHandle;
use sciter::{
    dom::{
        event::{BEHAVIOR_EVENTS, PHASE_MASK},
    },
    Element,
};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};
use tokio::sync::Mutex;

use crate::{players, services};

pub struct Player {
    rx: mpsc::Receiver<MediaControlEvent>,
    tx: mpsc::SyncSender<MediaControlEvent>,
    controls: Option<MediaControls>,
    sample_values: sciter::Value,
    player_service: Arc<Mutex<services::player::Player>>,
}

impl Player {
    pub fn new(player_service: Arc<Mutex<services::player::Player>>) -> Self {
        let (tx, rx): (
            mpsc::SyncSender<MediaControlEvent>,
            mpsc::Receiver<MediaControlEvent>,
        ) = mpsc::sync_channel(32);

        Self {
            sample_values: sciter::Value::new(),
            controls: None,
            rx,
            tx,
            player_service,
        }
    }

    fn fmt_time(&mut self, time: i32) -> String {
        format!("{}", players::FormatTime(Duration::from_secs(time as u64)))
    }

    pub async fn update_metadata(
        &mut self,
        title: String,
        album: String,
        artist: String,
        cover_url: String,
    ) {
        let lock = self.player_service.lock().await;

        self.controls
            .as_mut()
            .unwrap()
            .set_metadata(MediaMetadata {
                title: Some(&title),
                album: Some(&album),
                artist: Some(&artist),
                duration: lock.player.get_time(),
                cover_url: Some(&cover_url),
            })
            .unwrap();
    }

    fn handle_click(&mut self, id: &str, _element: Element) -> anyhow::Result<()> {
        match id {
            "play-pause" => {
                let psc = self.player_service.clone();
                tokio::spawn({
                    async move {
                        let mut ps = psc.lock().await;
                        let paused = ps.player.is_paused();

                        if paused {
                            ps.player.set_paused(false);
                        } else {
                            ps.player.set_paused(true);
                        }
                    }
                });
            }
            
            "back" => {
                let psc = self.player_service.clone();
                tokio::spawn({
                    async move {
                        let mut ps = psc.lock().await;
                        ps.prev()
                    }
                });
            }

            "next" => {
                let psc = self.player_service.clone();
                tokio::spawn({
                    async move {
                        let mut ps = psc.lock().await;
                        ps.next()
                    }
                });
            }

            _ => {}
        }

        bail!("Event not handled")
    }

    async fn set_paused(&mut self, state: bool) {
        let mut lock = self.player_service.lock().await;

        lock.player.set_paused(state);
        if state {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Paused {
                    progress: Some(souvlaki::MediaPosition(
                        lock.player.get_time().unwrap_or_default(),
                    )),
                })
                .ok();
        } else {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(
                        lock.player.get_time().unwrap_or_default(),
                    )),
                })
                .ok();
        }
    }
}

impl sciter::EventHandler for Player {
    fn on_event(
        &mut self,
        root: sciter::HELEMENT,
        _source: sciter::HELEMENT,
        target: sciter::HELEMENT,
        code: sciter::dom::event::BEHAVIOR_EVENTS,
        phase: sciter::dom::event::PHASE_MASK,
        _reason: sciter::dom::EventReason,
    ) -> bool {
        let event = self.rx.try_recv();
        let root = Element::from(root);

        let psc = self.player_service.clone();

        tokio::spawn({
            async move {
                let mut ps = psc.lock().await;
                let paused = ps.player.is_paused();
            }
        });

        

        if let Ok(event) = event {
            let psc = self.player_service.clone();

            tokio::spawn({
                async move {
                    let mut ps = psc.lock().await;
                    let paused = ps.player.is_paused();

                    match event {
                        MediaControlEvent::Toggle => ps.player.set_paused(!paused),
                        MediaControlEvent::Play => ps.player.set_paused(false),
                        MediaControlEvent::Pause => ps.player.set_paused(true),
                        MediaControlEvent::Stop => ps.player.stop(),
                        MediaControlEvent::SeekBy(_, duration) => ps.player.seek(duration),
                        _ => (),
                    }
                }
            });
        }

        match code {
            BEHAVIOR_EVENTS::BUTTON_CLICK => {
                let target = Element::from(target);
                let id = target.get_attribute("id");
                log::info!("{}", target);

                if id.is_some() && phase == PHASE_MASK::SINKING {
                    return self.handle_click(&id.unwrap(), target).is_ok();
                }
                false
            }

            BEHAVIOR_EVENTS::SELECT_VALUE_CHANGED => {
                let target = Element::from(target);
                if target.get_attribute("id").unwrap_or_default() == "audio-device"
                    && phase == PHASE_MASK::SINKING
                {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn document_complete(&mut self, root: sciter::HELEMENT, _target: sciter::HELEMENT) {
        let root = Element::from(root);

        #[cfg(not(target_os = "windows"))]
        let hwnd = None;

        #[cfg(target_os = "windows")]
        let hwnd = {
            use raw_window_handle::RawWindowHandle;

            let mut h = Win32WindowHandle::empty();
            h.hwnd = root.get_hwnd(true) as *mut _;

            let handle = match RawWindowHandle::Win32(h) {
                RawWindowHandle::Win32(h) => h,
                _ => unreachable!(),
            };
            Some(handle.hwnd)
        };

        log::debug!("Window handle: {:?}", hwnd);

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
}

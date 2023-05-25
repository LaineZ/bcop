use std::{sync::mpsc, time::Duration};

use raw_window_handle::Win32WindowHandle;
use sciter::{dispatch_script_call, make_args, Element, Value};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};

use crate::players::{self, bass::BassPlayer, internal::InternalPlayer, AudioSystem};

pub struct Player {
    player: Box<dyn players::Player>,
    selected_audiosystem: AudioSystem,
    event: sciter::Value,
    rx: mpsc::Receiver<MediaControlEvent>,
    tx: mpsc::SyncSender<MediaControlEvent>,
    controls: Option<MediaControls>,
}

impl Player {
    pub fn new(backend: AudioSystem) -> Self {
        let (tx, rx): (
            mpsc::SyncSender<MediaControlEvent>,
            mpsc::Receiver<MediaControlEvent>,
        ) = mpsc::sync_channel(32);

        match backend {
            AudioSystem::Internal => {
                return Self {
                    controls: None,
                    rx,
                    tx,
                    event: sciter::Value::new(),
                    player: Box::new(InternalPlayer::new()),
                    selected_audiosystem: AudioSystem::Internal,
                }
            }
            AudioSystem::Bass => {
                return if let Ok(bass) = BassPlayer::new() {
                    Self {
                        controls: None,
                        rx,
                        tx,
                        event: sciter::Value::new(),
                        player: Box::new(bass),
                        selected_audiosystem: AudioSystem::Bass,
                    }
                } else {
                    return Self {
                        controls: None,
                        rx,
                        tx,
                        event: sciter::Value::new(),
                        player: Box::new(InternalPlayer::new()),
                        selected_audiosystem: AudioSystem::Internal,
                    };
                }
            }
        }
    }

    fn switch_backend(&mut self, backend: i32) -> bool {
        match backend {
            0 => {
                self.player.stop();
                self.player = Box::new(InternalPlayer::new());
                true
            }
            1 => {
                if let Ok(bass) = BassPlayer::new() {
                    self.player.stop();
                    self.player = Box::new(bass);
                    true
                } else {
                    false
                }
            }
            _ => {
                log::error!("Invalid backend value out of range 1 < {}", backend);
                false
            }
        }
    }

    fn set_state_change_callback(&mut self, value: sciter::Value) {
        self.event = value;
    }

    fn fmt_time(&mut self, time: i32) -> String {
        format!("{}", players::FormatTime(Duration::from_secs(time as u64)))
    }

    pub fn load_track(&mut self, url: String) -> bool {
        let res = self.player.switch_track(url).is_ok();
        self.event.call(None, &make_args!(""), None).unwrap();
        res
    }

    pub fn update_metadata(
        &mut self,
        title: String,
        album: String,
        artist: String,
        cover_url: String,
    ) {
        self.controls
            .as_mut()
            .unwrap()
            .set_metadata(MediaMetadata {
                title: Some(&title),
                album: Some(&album),
                artist: Some(&artist),
                duration: self.player.get_time(),
                cover_url: Some(&cover_url),
            })
            .unwrap();
    }

    fn set_paused(&mut self, state: bool) {
        self.player.set_paused(state);
        if state {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Paused {
                    progress: Some(souvlaki::MediaPosition(
                        self.player.get_time().unwrap_or_default(),
                    )),
                }).ok();
        } else {
            self.controls
                .as_mut()
                .unwrap()
                .set_playback(MediaPlayback::Playing {
                    progress: Some(souvlaki::MediaPosition(
                        self.player.get_time().unwrap_or_default(),
                    )),
                }).ok();
        }
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    fn stop(&mut self) {
        self.player.stop();
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    fn seek(&mut self, seconds: i32) {
        self.player.seek(Duration::from_secs(seconds as u64));
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    fn get_time(&mut self) -> i32 {
        let time = self.player.get_time().unwrap_or_default();
        time.as_secs() as i32
    }

    fn restart_player_on_fault(&mut self) {
        if !self.player.is_initialized() {
            log::warn!("Restarting player thread");
            self.switch_backend(self.selected_audiosystem as i32);
        }
    }

    fn get_volume(&mut self) -> i32 {
        self.player.get_volume() as i32
    }

    fn set_volume(&mut self, value: i32) {
        self.player.set_volume(value as u16);
    }

    fn force_update(&self) {
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    fn get_samples(&mut self) -> Value {
        let mut value = Value::new();

        for sample in self.player.get_samples() {
            value.push(sample as f64);
        }

        value
    }
}

impl sciter::EventHandler for Player {
    dispatch_script_call! {
        fn load_track(String);
        fn set_paused(bool);
        fn is_paused();
        fn stop();
        fn seek(i32);
        fn get_time();
        fn fmt_time(i32);
        fn switch_backend(i32);
        fn set_state_change_callback(Value);
        fn get_volume();
        fn set_volume(i32);
        fn force_update();
        fn restart_player_on_fault();
        fn update_metadata(String, String, String, String);
        fn get_samples();
    }

    fn on_event(
        &mut self,
        root: sciter::HELEMENT,
        _source: sciter::HELEMENT,
        _target: sciter::HELEMENT,
        _code: sciter::dom::event::BEHAVIOR_EVENTS,
        _phase: sciter::dom::event::PHASE_MASK,
        _reason: sciter::dom::EventReason,
    ) -> bool {
        let event = self.rx.try_recv();
        let root = Element::from(root);


        if let Ok(event) = event {
            match event {
                MediaControlEvent::Toggle => self.set_paused(!self.is_paused()),
                MediaControlEvent::Play => self.set_paused(false),
                MediaControlEvent::Pause => self.set_paused(true),
                MediaControlEvent::Stop => self.stop(),
                MediaControlEvent::SeekBy(_, duration) => self.player.seek(duration),
                _ => (),
            }
        }

        root.call_function("update", &make_args!(""))
        .unwrap();
        false
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

use std::{sync::mpsc, time::Duration};

use raw_window_handle::Win32WindowHandle;
use sciter::{dispatch_script_call, make_args, Element, Value, dom::{self, event::{BEHAVIOR_EVENTS, PHASE_MASK}}};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig};

use crate::players::{self, bass::BassPlayer, AudioSystem};

pub struct Player {
    player: Box<dyn players::Player>,
    _selected_audiosystem: AudioSystem,
    event: sciter::Value,
    rx: mpsc::Receiver<MediaControlEvent>,
    tx: mpsc::SyncSender<MediaControlEvent>,
    controls: Option<MediaControls>,
    sample_values: sciter::Value,
}

impl Player {
    pub fn new(backend: AudioSystem, device_id: usize) -> Self {
        let (tx, rx): (
            mpsc::SyncSender<MediaControlEvent>,
            mpsc::Receiver<MediaControlEvent>,
        ) = mpsc::sync_channel(32);

        let bass = BassPlayer::new(device_id).expect("Unable to initialize bass library");

        match backend {
            AudioSystem::Bass => {
                Self {
                    sample_values: sciter::Value::new(),
                    controls: None,
                    rx,
                    tx,
                    event: sciter::Value::new(),
                    player: Box::new(bass),
                    _selected_audiosystem: AudioSystem::Bass,
                }
            }
        }
    }

    fn switch_backend(&mut self, backend: i32, device_id: i32) -> bool {
        match backend {
            0 => {
                if let Ok(bass) = BassPlayer::new(device_id as usize) {
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
        self.sample_values.clear();
        for sample in self.player.get_samples() {
            self.sample_values.push(*sample as f64);
        }

        let new_value = std::mem::take(&mut self.sample_values);

        new_value
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
        fn switch_backend(i32, i32);
        fn set_state_change_callback(Value);
        fn get_volume();
        fn set_volume(i32);
        fn force_update();
        fn update_metadata(String, String, String, String);
        fn get_samples();
    }

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

        match code {
            BEHAVIOR_EVENTS::SELECT_VALUE_CHANGED => {
                let target = Element::from(target);
                if target.get_attribute("id").unwrap_or_default() == "audio-device" && phase == PHASE_MASK::SINKING {
                    let id = target.child(1).unwrap().get_attribute("value").unwrap_or_default();
                    self.player.switch_device(id.parse().unwrap_or_default()).unwrap_or_else(|op| {
                        log::error!("Unable to switch audio device: {}", op);
                        root.call_function("showErrorModal", &make_args!(&format!("Unable to switch audio device: {}", op)))
                        .unwrap();
                    });
                    return true
                }
                false
            }
            _ => {
                false
            },
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

        // populate options
        let mut audio_device_dropdown = root.find_first("#audio-device").unwrap().unwrap();
        let mut html = String::from("");
        for (idx, dev) in self.player.get_devices().iter().enumerate() {
            html += &format!("<option value=\"{}\" class=\"audio-device-option\">{}</option>", idx, dev);
        }
        audio_device_dropdown.set_html(html.as_bytes(), Some(dom::SET_ELEMENT_HTML::SIH_REPLACE_CONTENT)).unwrap();
    }
}

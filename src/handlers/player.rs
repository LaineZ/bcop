use std::time::Duration;

use sciter::{dispatch_script_call, make_args, Value};

use crate::players::{
    self,
    bass::BassPlayer,
    internal::{self, InternalPlayer},
    AudioSystem,
};

pub struct Player {
    player: Box<dyn players::Player>,
    selected_audiosystem: AudioSystem,
    event: sciter::Value,
}

impl Player {
    pub fn new(backend: AudioSystem) -> Self {
        match backend {
            AudioSystem::Internal => return Self {
                event: sciter::Value::new(),
                player: Box::new(InternalPlayer::new()),
                selected_audiosystem: AudioSystem::Internal
            },
            AudioSystem::Bass => {
                return if let Ok(bass) = BassPlayer::new() {
                    Self {
                        event: sciter::Value::new(),
                        player: Box::new(bass),
                        selected_audiosystem: AudioSystem::Bass
                    }
                } else {
                    return Self {
                        event: sciter::Value::new(),
                        player: Box::new(InternalPlayer::new()),
                        selected_audiosystem: AudioSystem::Internal
                    }
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
            },
            1 => {
                if let Ok(bass) = BassPlayer::new() {
                    self.player.stop();
                    self.player = Box::new(bass);
                    true
                } else {
                    false
                }
            },
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

    fn set_paused(&mut self, state: bool) {
        self.player.set_paused(state);
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    fn is_paused(&mut self) -> bool {
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
    }
}

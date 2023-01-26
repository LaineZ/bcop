use std::time::Duration;

use sciter::{dispatch_script_call, make_args, Value};

use crate::playback::FormatTime;

pub struct Player {
    player: crate::playback::Player,
    event: sciter::Value,
}

impl Player {
    pub fn new() -> Self {
        Self {
            player: crate::playback::Player::new(),
            event: sciter::Value::new(),
        }
    }

    pub fn set_state_change_callback(&mut self, value: sciter::Value) {
        log::info!("Handler installed");
        self.event = value;
    }

    pub fn fmt_time(&mut self, time: i32) -> String {
        format!("{}", FormatTime(Duration::from_secs(time as u64)))
    }

    pub fn load_track(&mut self, url: String) {
        self.player.switch_track(url);
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    pub fn set_paused(&mut self, state: bool) {
        self.player.set_paused(state);
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    pub fn is_paused(&mut self) -> bool {
        self.player.is_paused()
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    pub fn seek(&mut self, seconds: i32) {
        self.player.seek(Duration::from_secs(seconds as u64));
        self.event.call(None, &make_args!(""), None).unwrap();
    }

    pub fn get_time(&mut self) -> i32 {
        let time = self.player.get_time().unwrap_or(Duration::from_secs(0));
        time.as_secs() as i32
    }

    pub fn get_volume(&mut self) -> i32 {
        self.player.get_volume() as i32
    }

    pub fn set_volume(&mut self, value: i32) {
        self.player.set_volume(value as u16);
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
        fn set_state_change_callback(Value);
        fn get_volume();
        fn set_volume(i32);
    }
}

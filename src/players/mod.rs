pub mod internal;
pub mod bass;

use std::{time::Duration, fmt::{Display, self}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum AudioSystem {
    Internal = 0,
    Bass = 1,
}

pub trait Player {
    fn restart_on_fault(&self) -> bool;
    fn get_time(&self) -> Option<Duration>;
    fn is_playing(&self) -> bool;
    fn is_paused(&self) -> bool;
    fn set_paused(&mut self, paused: bool);
    fn set_volume(&mut self, value: u16);
    fn get_volume(&mut self) -> u16;
    fn stop(&mut self);
    fn switch_track(&mut self, url: String) -> anyhow::Result<()>;
    fn seek(&mut self, time: Duration);
}

pub struct FormatTime(pub Duration);

impl Display for FormatTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_secs = self.0.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        write!(f, "{:02}:{:02}", mins, secs)
    }
}
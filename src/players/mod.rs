pub mod bass;

use std::{time::Duration, fmt::{Display, self}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum AudioSystem {
    Bass = 0,
}

pub trait Player {
    fn is_initialized(&self) -> bool;
    fn get_time(&self) -> Option<Duration>;
    fn get_devices(&self) -> Vec<String>;
    fn switch_device(&mut self, index: usize) -> anyhow::Result<()>;
    fn is_playing(&self) -> bool;
    fn is_paused(&self) -> bool;
    fn set_paused(&mut self, paused: bool);
    fn set_volume(&mut self, value: u16);
    fn get_volume(&mut self) -> u16;
    fn stop(&mut self);
    fn switch_track(&mut self, url: String) -> anyhow::Result<()>;
    fn seek(&mut self, time: Duration);
    fn get_samples(&mut self) -> &[f32];
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
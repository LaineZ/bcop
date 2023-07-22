use serde::{Deserialize, Serialize};

use crate::players::AudioSystem;

const LOAD_ARTWORKS: [ArtworkThumbnailQuality; 5] = [
    ArtworkThumbnailQuality::VeryHigh,
    ArtworkThumbnailQuality::High,
    ArtworkThumbnailQuality::Medium,
    ArtworkThumbnailQuality::Low,
    ArtworkThumbnailQuality::VeryLow,
];

const AUDIO_SYSTEM: [AudioSystem; 1] = [AudioSystem::Bass];

/// Artwork quality.
/// Bandcamp returns artworks in different formats and resolutions. This can be set with number in URL
/// https://f4.bcbits.com/img/a<ART_ID>_<RESOLUTION>.jpg
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum ArtworkThumbnailQuality {
    VeryHigh = 5,
    High = 7,
    Medium = 6,
    Low = 42,
    VeryLow = 22,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Default for WindowGeometry {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 900,
            h: 600,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub load_artworks: ArtworkThumbnailQuality,
    pub window_geometry: WindowGeometry,
    pub volume: u16,
    pub save_queue_on_exit: bool,
    pub theme_name: String,
    pub audio_system: AudioSystem,
    pub device_index: usize,
    pub visualizer: bool,
}

impl Config {
    pub fn new() -> Self {
        // trying to load
        if let Ok(config) = std::fs::read_to_string("configuration.toml") {
            if let Ok(config) = toml::from_str::<Config>(&config) {
                return config;
            } else {
                log::warn!("Unable to parse config file; Using defaults");
            }
        } else {
            log::warn!("Unable to load config file; Using defaults");
        }

        Self {
            load_artworks: ArtworkThumbnailQuality::High,
            volume: 100,
            window_geometry: WindowGeometry::default(),
            save_queue_on_exit: true,
            theme_name: String::from("hope_diamond"),
            audio_system: AudioSystem::Bass,
            device_index: 0,
            visualizer: true,
        }
    }

    pub fn save_config(&self) {
        std::fs::write(
            "configuration.toml",
            toml::to_string(&self).unwrap_or_default(),
        )
        .unwrap_or_else(|op| {
            log::warn!("Unable to save configuration file: {}", op);
        });

        log::info!("Settings saved");
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

use sciter::{dispatch_script_call, Element, Value};
use serde::{Deserialize, Serialize};

const PROXY_TYPE: [ProxyType; 3] = [ProxyType::None, ProxyType::UseHttp, ProxyType::UseProxy];

const LOAD_ARTWORKS: [ArtworkThumbnailQuality; 4] = [
    ArtworkThumbnailQuality::High,
    ArtworkThumbnailQuality::Medium,
    ArtworkThumbnailQuality::Low,
    ArtworkThumbnailQuality::VeryLow,
];

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ProxyType {
    None = 0,
    UseHttp = 1,
    UseProxy = 2,
}

/// Artwork quality.
/// Bandcamp returns artworks in different resolutions. This can be set with number in URL
/// https://f4.bcbits.com/img/a<ART_ID>_<RESOLUTION>.jpg
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum ArtworkThumbnailQuality {
    High = 7,
    Medium = 6,
    Low = 42,
    VeryLow = 22,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    proxy_type: ProxyType,
    load_artworks: ArtworkThumbnailQuality,
    volume: u16,
}

impl Config {
    pub fn default() -> Self {
        Self {
            load_artworks: ArtworkThumbnailQuality::High,
            proxy_type: ProxyType::None,
            volume: 100,
        }
    }

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
            proxy_type: ProxyType::None,
            volume: 100,
        }
    }

    pub fn populate_settings(&self, settings_window: Element) {
        let mut proxy_dropdown = settings_window.find_first("#use-proxy").unwrap().unwrap();
        let mut load_artworks_dropdown = settings_window
            .find_first("#artwork-quality")
            .unwrap()
            .unwrap();

        proxy_dropdown.set_value(self.proxy_type as i32).unwrap();
        load_artworks_dropdown
            .set_value(
                LOAD_ARTWORKS
                    .iter()
                    .position(|&v| v == self.load_artworks)
                    .unwrap_or(0) as i32,
            )
            .unwrap();
    }

    pub fn setup_volume(&self, mut volume_track: Element) {
        volume_track.set_value(self.volume as i32).unwrap();
    }

    pub fn store_volume(&mut self, volume_track: Element) {
        let track_value = volume_track
            .get_value()
            .to_string()
            .replace("\"", "")
            .parse::<i32>()
            .unwrap_or(100);

        self.volume = track_value as u16;
    }

    pub fn set_settings(&mut self, settings_window: Element) {
        let proxy_dropdown = settings_window.find_first("#use-proxy").unwrap().unwrap();
        let load_artworks_dropdown = settings_window
            .find_first("#artwork-quality")
            .unwrap()
            .unwrap();

        let proxy_value = proxy_dropdown
            .get_value()
            .to_string()
            .replace("\"", "")
            .parse::<i32>()
            .unwrap_or(0);
        let load_artworks_value = load_artworks_dropdown
            .get_value()
            .to_string()
            .replace("\"", "")
            .parse::<i32>()
            .unwrap_or(0);

        self.proxy_type = if PROXY_TYPE.len() - 1 < proxy_value as usize {
            log::warn!(
                "Value out of range {} > {}",
                proxy_value,
                PROXY_TYPE.len() - 1
            );
            ProxyType::None
        } else {
            PROXY_TYPE[proxy_value as usize]
        };

        self.load_artworks = if LOAD_ARTWORKS.len() - 1 < load_artworks_value as usize {
            log::warn!(
                "Value out of range {} > {}",
                load_artworks_value,
                LOAD_ARTWORKS.len() - 1
            );
            ArtworkThumbnailQuality::High
        } else {
            LOAD_ARTWORKS[load_artworks_value as usize]
        };
    }

    pub fn save_config(&self) {
        std::fs::write(
            "configuration.toml",
            toml::to_string(&self).unwrap_or(String::new()),
        )
        .unwrap_or_else(|op| {
            log::warn!("Unable to save configuration file: {}", op);
        });

        log::info!("Settings saved");
    }

    pub fn get_proxy(&self) -> i32 {
        self.proxy_type as i32
    }

    pub fn get_load_artworks(&self) -> i32 {
        self.load_artworks as i32
    }
}

impl sciter::EventHandler for Config {
    dispatch_script_call! {
        fn get_proxy();
        fn get_load_artworks();
        fn populate_settings(Value);
        fn set_settings(Value);
        fn save_config();
        fn setup_volume(Value);
        fn store_volume(Value);
    }
}

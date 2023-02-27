use sciter::{
    dispatch_script_call,
    dom::{
        self,
        event::{self, BEHAVIOR_EVENTS},
    },
    make_args, Element, Value,
};
use serde::{Deserialize, Serialize};

const LOAD_ARTWORKS: [ArtworkThumbnailQuality; 5] = [
    ArtworkThumbnailQuality::VeryHigh,
    ArtworkThumbnailQuality::High,
    ArtworkThumbnailQuality::Medium,
    ArtworkThumbnailQuality::Low,
    ArtworkThumbnailQuality::VeryLow,
];

/// Artwork quality.
/// Bandcamp returns artworks in different resolutions. This can be set with number in URL
/// https://f4.bcbits.com/img/a<ART_ID>_<RESOLUTION>.jpg
#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum ArtworkThumbnailQuality {
    VeryHigh = 5,
    High = 7,
    Medium = 6,
    Low = 42,
    VeryLow = 22,
}

#[derive(Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Config {
    load_artworks: ArtworkThumbnailQuality,
    volume: u16,
    tag_pane_hidden: bool,
    theme_name: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            load_artworks: ArtworkThumbnailQuality::High,
            volume: 100,
            tag_pane_hidden: false,
            theme_name: String::from("hope_diamond"),
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
            tag_pane_hidden: false,
            volume: 100,
            theme_name: String::from("hope_diamond"),
        }
    }

    pub fn set_settings(&mut self, settings_window: Element) {
        let load_artworks_dropdown = settings_window
            .find_first("#artwork-quality")
            .unwrap()
            .unwrap();

        let theme_dropdown = settings_window.find_first("#theme").unwrap().unwrap();
        let load_artworks_value = load_artworks_dropdown
            .get_value()
            .to_string()
            .replace('\"', "")
            .parse::<i32>()
            .unwrap_or(0);

        let theme_value = theme_dropdown.get_value().to_string().replace('\"', "");

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

        self.theme_name = if !theme_value.trim().is_empty() {
            theme_value
        } else {
            log::warn!("Invalid theme string: `{}`", theme_value);
            "hope_diamond".to_string()
        };
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

    pub fn get_load_artworks(&self) -> i32 {
        self.load_artworks as i32
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl sciter::EventHandler for Config {
    fn document_complete(&mut self, root: sciter::HELEMENT, _target: sciter::HELEMENT) {
        let root = Element::from(root);

        // setting volume
        let mut volume_bar = root.find_first("#volume").unwrap().unwrap();
        volume_bar.set_value(self.volume as i32).unwrap();

        // setting tag visiability
        let mut tags = root.find_first("#tags-select").unwrap().unwrap();
        if self.tag_pane_hidden {
            tags.set_attribute("class", "closed").unwrap();
        }

        // populate settings
        let mut load_artworks_dropdown = root.find_first("#artwork-quality").unwrap().unwrap();
        let mut theme_dropdown = root.find_first("#theme").unwrap().unwrap();
        theme_dropdown.set_value(&self.theme_name).unwrap();
        load_artworks_dropdown
            .set_value(
                LOAD_ARTWORKS
                    .iter()
                    .position(|&v| v == self.load_artworks)
                    .unwrap_or(0) as i32,
            )
            .unwrap();

        root.call_function("setTheme", &make_args!(&self.theme_name))
            .unwrap();
    }

    fn on_event(
        &mut self,
        root: sciter::HELEMENT,
        _source: sciter::HELEMENT,
        target: sciter::HELEMENT,
        code: event::BEHAVIOR_EVENTS,
        _phase: event::PHASE_MASK,
        _reason: dom::EventReason,
    ) -> bool {
        match code {
            BEHAVIOR_EVENTS::BUTTON_CLICK => {
                let target = Element::from(target);

                if let Some(id) = target.get_attribute("id") {
                    if id == "volume" {
                        let track_value = target
                            .get_value()
                            .to_string()
                            .replace('\"', "")
                            .parse::<i32>()
                            .unwrap_or(100);

                        self.volume = track_value as u16;
                        return true;
                    }
                }
                false
            }

            BEHAVIOR_EVENTS::DOCUMENT_CLOSE_REQUEST => {
                let root = Element::from(root);
                let tags = root.find_first("#tags-select").unwrap().unwrap();
                self.tag_pane_hidden = tags
                    .get_attribute("class")
                    .unwrap_or_default()
                    .contains("closed");
                self.save_config();
                false
            }
            _ => false,
        }
    }

    dispatch_script_call! {
        fn get_load_artworks();
        fn set_settings(Value);
        fn save_config();
    }
}

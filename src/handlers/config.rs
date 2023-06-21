use sciter::{
    dispatch_script_call,
    dom::{
        self,
        event::{self, BEHAVIOR_EVENTS, PHASE_MASK},
    },
    make_args,
    window::Rectangle,
    Element, Value,
};
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

/// Converts integer value to enum
macro_rules! set_enum {
    ($arr:expr, $idx:expr) => {
        match $arr.get($idx as usize) {
            Some(val) => *val,
            None => {
                log::warn!("Value out of range {} > {}", $idx, $arr.len() - 1);
                $arr[0]
            }
        }
    };
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindowGeometry {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl From<Rectangle> for WindowGeometry {
    fn from(value: Rectangle) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.width,
            h: value.height,
        }
    }
}

impl Into<Rectangle> for WindowGeometry {
    fn into(self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.w,
            height: self.h,
        }
    }
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
    load_artworks: ArtworkThumbnailQuality,
    pub window_geometry: WindowGeometry,
    volume: u16,
    save_queue_on_exit: bool,
    theme_name: String,
    audio_system: AudioSystem,
    device_index: usize,
    visualizer: bool,
}

fn set_widget_state<S: AsRef<str>, I: Into<Value>>(root: &Element, selector: S, value: I) {
    let mut element = root.find_first(selector.as_ref()).unwrap().unwrap();
    element.set_value(value).unwrap();
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

    pub fn set_settings(&mut self, settings_window: Element) {
        let load_artworks_dropdown = settings_window
            .find_first("#artwork-quality")
            .unwrap()
            .unwrap();
        let theme_dropdown = settings_window.find_first("#theme").unwrap().unwrap();
        let audio_system_dropdown = settings_window
            .find_first("#audio-backend")
            .unwrap()
            .unwrap();
        let save_queue_on_exit = settings_window
            .find_first("#save-queue-on-exit")
            .unwrap()
            .unwrap();

        let visualizer = settings_window.find_first("#visualizer").unwrap().unwrap();

        let load_artworks_value = load_artworks_dropdown
            .get_value()
            .to_string()
            .replace('\"', "")
            .parse::<i32>()
            .unwrap_or(0);

        let audio_backend_value = audio_system_dropdown
            .get_value()
            .to_string()
            .replace('\"', "")
            .parse::<i32>()
            .unwrap_or(0);

        let theme_value = theme_dropdown.get_value().to_string().replace('\"', "");
        self.load_artworks = set_enum!(LOAD_ARTWORKS, load_artworks_value);
        self.save_queue_on_exit = save_queue_on_exit.get_value().to_bool().unwrap_or(true);
        self.visualizer = visualizer.get_value().to_bool().unwrap_or(true);
        self.audio_system = set_enum!(AUDIO_SYSTEM, audio_backend_value);

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

    pub fn get_audio_system(&self) -> AudioSystem {
        self.audio_system
    }

    pub fn get_save_queue_on_exit(&self) -> bool {
        self.save_queue_on_exit
    }

    pub fn get_visualizer(&self) -> bool {
        self.visualizer
    }

    pub fn set_geometry(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.window_geometry.x = x;
        self.window_geometry.y = y;
        self.window_geometry.w = w;
        self.window_geometry.h = h;
    }

    pub fn get_audio_device_index(&self) -> usize {
        self.device_index
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

        // populate settings
        let mut load_artworks_dropdown = root.find_first("#artwork-quality").unwrap().unwrap();
        let mut theme_dropdown = root.find_first("#theme").unwrap().unwrap();
        let mut audio_system_dropdown = root.find_first("#audio-backend").unwrap().unwrap();
        let mut audio_device_dropdown = root.find_first("#audio-device").unwrap().unwrap();
        audio_device_dropdown
            .set_value(self.device_index as i32)
            .unwrap();

        theme_dropdown.set_value(&self.theme_name).unwrap();

        set_widget_state(&root, "#save-queue-on-exit", self.save_queue_on_exit);
        set_widget_state(&root, "#visualizer", self.visualizer);

        load_artworks_dropdown
            .set_value(
                LOAD_ARTWORKS
                    .iter()
                    .position(|&v| v == self.load_artworks)
                    .unwrap_or(0) as i32,
            )
            .unwrap();

        audio_system_dropdown
            .set_value(
                AUDIO_SYSTEM
                    .iter()
                    .position(|&v| v == self.audio_system)
                    .unwrap_or(0) as i32,
            )
            .unwrap();

        root.call_function("setTheme", &make_args!(&self.theme_name))
            .unwrap();
    }

    fn on_event(
        &mut self,
        _root: sciter::HELEMENT,
        _source: sciter::HELEMENT,
        target: sciter::HELEMENT,
        code: event::BEHAVIOR_EVENTS,
        phase: event::PHASE_MASK,
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

            BEHAVIOR_EVENTS::SELECT_VALUE_CHANGED => {
                let target = Element::from(target);
                if target.get_attribute("id").unwrap_or_default() == "audio-device"
                    && phase == PHASE_MASK::SINKING
                {
                    let id = target
                        .child(1)
                        .unwrap()
                        .get_attribute("value")
                        .unwrap_or_default();
                    self.device_index = id.parse().unwrap_or_default();
                }
                false
            }

            BEHAVIOR_EVENTS::DOCUMENT_CLOSE_REQUEST => {
                self.save_config();
                false
            }
            _ => false,
        }
    }

    dispatch_script_call! {
        fn get_load_artworks();
        fn get_save_queue_on_exit();
        fn get_visualizer();
        fn set_settings(Value);
        fn set_geometry(i32, i32, i32, i32);
        fn save_config();
    }
}

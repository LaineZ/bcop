use sciter::{
    dispatch_script_call,
    dom::{
        self,
        event::{self, BEHAVIOR_EVENTS, PHASE_MASK},
    },
    make_args,
    Element, Value,
};

fn set_widget_state<S: AsRef<str>, I: Into<Value>>(root: &Element, selector: S, value: I) {
    let mut element = root.find_first(selector.as_ref()).unwrap().unwrap();
    element.set_value(value).unwrap();
}

impl Config {
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

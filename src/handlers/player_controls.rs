use std::time::Duration;

use anyhow::bail;
use sciter::{
    dom::event::{BEHAVIOR_EVENTS, PHASE_MASK},
    Element,
};

use crate::{players, services};

pub struct PlayerControls<'a> {
    player_service: &'a mut services::player::Player,
    controls: Element,
}

impl<'a> PlayerControls<'a> {
    pub fn new(player_service: &'a mut services::player::Player) -> Self {
        Self {
            player_service,
            controls: Element::create("a").unwrap(),
        }
    }

    fn fmt_time(&mut self, time: i32) -> String {
        format!("{}", players::FormatTime(Duration::from_secs(time as u64)))
    }

    fn handle_click(&mut self, id: &str, _element: Element) -> anyhow::Result<()> {
        match id {
            "play-pause" => {
                self.player_service
                    .set_paused(!self.player_service.player.is_paused());
            }

            "back" => {
                self.player_service.prev()?;
            }

            "next" => {
                self.player_service.next()?;
            }

            _ => {}
        }

        bail!("Event not handled")
    }

    fn update_control(&self, element_root: Element) {
        let player_controls = element_root.find_first("#controls").unwrap().unwrap();
        let mut track_name = element_root
            .find_first("#track-info-name")
            .unwrap()
            .unwrap();

        if self.player_service.queue.is_empty() {
            // disable all controls if queuelist is empty
            player_controls.children().into_iter().for_each(|mut f| {
                f.set_attribute("disabled", "true").unwrap();
            });
        }

        if let Some(track) = self.player_service.get_current_track() {

        }

        track_name.set_text("").unwrap();
    }
}

impl sciter::EventHandler for PlayerControls<'_> {
    fn on_event(
        &mut self,
        root: sciter::HELEMENT,
        _source: sciter::HELEMENT,
        target: sciter::HELEMENT,
        code: sciter::dom::event::BEHAVIOR_EVENTS,
        phase: sciter::dom::event::PHASE_MASK,
        _reason: sciter::dom::EventReason,
    ) -> bool {
        self.player_service.process_mediabutton_events();
        match code {
            BEHAVIOR_EVENTS::BUTTON_CLICK => {
                let target = Element::from(target);
                let id = target.get_attribute("id");
                log::info!("{}", target);

                if id.is_some() && phase == PHASE_MASK::SINKING {
                    return self.handle_click(&id.unwrap(), target).is_ok();
                }
                false
            }

            BEHAVIOR_EVENTS::SELECT_VALUE_CHANGED => {
                let target = Element::from(target);
                if target.get_attribute("id").unwrap_or_default() == "audio-device"
                    && phase == PHASE_MASK::SINKING
                {
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn document_complete(&mut self, root: sciter::HELEMENT, _target: sciter::HELEMENT) {
        let root = Element::from(root);
        self.update_control(root);
    }
}
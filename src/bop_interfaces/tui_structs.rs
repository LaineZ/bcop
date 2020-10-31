use crate::{bc_core::album_parsing, model::discover};
use std::fmt::Display;

#[derive(Clone)]
pub struct State {
    /// Used only in INTERNAL DRAWING FUNCTIONS if you want to set error/information you need set with specified functions
    pub header_text: String,
    /// Used only in INTERNAL DRAWING FUNCTIONS if you want to set error/information you need set with specified functions
    pub bottom_text: String,
    /// Used only in INTERNAL DRAWING FUNCTIONS
    pub error: bool,
    loaded_discover_pages: usize,
    pub current_view: usize,
    pub selected_tags: Vec<String>,
    pub discover: Vec<discover::Item>,
}

impl State {
    pub fn new() -> Self {
        Self {
            header_text: "Welcome!".to_string(),
            bottom_text: "No player".to_string(),
            error: false,
            current_view: 0,
            loaded_discover_pages: 0,
            selected_tags: Vec::new(),
            discover: Vec::new(),
        }
    }

    pub fn error<T: Display>(&mut self, item: &T) {
        self.error = true;
        self.header_text = item.to_string();
    }

    pub fn information<T: Display>(&mut self, item: &T) {
        self.error = false;
        self.header_text = item.to_string();
    }

    pub fn extend_discover(&mut self) -> Result<(), anyhow::Error> {
        if self.selected_tags.len() > 0 {
            self.loaded_discover_pages += 1;
            log::info!("Loading discover: {}", self.loaded_discover_pages);
            let discover = album_parsing::get_tag_data(
                self.selected_tags.clone(),
                self.loaded_discover_pages,
            )?
            .items;
            self.discover.extend(discover);
        }

        Ok(())
    }
}

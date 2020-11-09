use crate::{bc_core::album_parsing, model::discover};
use std::fmt::Display;

#[derive(Clone)]
pub struct State {
    loaded_discover_pages: usize,
    pub selected_tags: Vec<String>,
    pub discover: Vec<discover::Item>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loaded_discover_pages: 0,
            selected_tags: Vec::new(),
            discover: Vec::new(),
        }
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

    pub fn add_tag(&mut self, tag: String) {
        let is_present = self.selected_tags.iter().any(|s| s == &tag);
        if !is_present {
            self.selected_tags.push(tag);
        }
    }
}

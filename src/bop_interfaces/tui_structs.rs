use crate::{bc_core::album_parsing, model::discover};
use std::fmt::Display;

#[derive(Clone)]
pub struct State {
    loaded_discover_pages: usize,
    pub discover: Vec<discover::Item>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loaded_discover_pages: 0,
            discover: Vec::new(),
        }
    }

    pub fn extend_discover(&mut self, tags: Vec<String>) -> Result<(), anyhow::Error> {
        log::info!("Selected tags: {}", tags.join(", "));
        if !tags.is_empty() {
            self.loaded_discover_pages += 1;
            log::info!("Loading discover: {}", self.loaded_discover_pages);
            let discover = album_parsing::get_tag_data(tags, self.loaded_discover_pages)?.items;
            self.discover.extend(discover);
        } else {
            log::warn!("Failed to load discover for some reasons...");
        }

        Ok(())
    }
}

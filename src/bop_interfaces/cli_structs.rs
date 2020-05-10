use crate::model::discover;
use anyhow::Result;
use crossterm::{cursor, style::Print, terminal::size, QueueableCommand};
#[derive(PartialEq, Clone)]
pub enum CurrentView {
    Albums,
    Tags,
    Queue,
    Diagnositcs
}

#[derive(Clone)]
pub struct ListBoxTag {
    pub content: Vec<String>,
    pub selected_idx: usize,
    pub selected_page: usize,
    pub selected_tag_name: String,
}

#[derive(Clone)]
pub struct ListBoxDiscover {
    pub content: Vec<discover::Item>,
    pub selected_idx: usize,
    pub selected_page: usize,
    pub loadedpages: i32,
}

#[derive(Clone)]
pub struct ListBoxQueue {
    pub content: Vec<QueuedTrack>,
    pub selected_idx: usize,
    pub selected_page: usize,
}

#[derive(Clone)]
pub struct QueuedTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub audio_url: String,
    pub duration: f64,
}

#[derive(Clone)]
pub struct Diagnositcs {
    pub content: Vec<String>,
    pub selected_idx: usize,
    pub selected_page: usize,
}

#[derive(Clone)]
pub struct State {
    pub statusbar_text: String,
    pub bottom_text: String,
    pub error: bool,
    pub current_view: CurrentView,
    pub discover: ListBoxDiscover,
    pub selected_tags: Vec<String>,
    pub tags: ListBoxTag,
    pub queue: ListBoxQueue,
    pub queue_pos: usize,
    pub display_tags: bool,
    pub diagnostics: Diagnositcs,
    pub is_paused: bool,
}

impl Default for ListBoxTag {
    fn default() -> Self {
        ListBoxTag {
            content: Vec::new(),
            selected_idx: 0,
            selected_page: 0,
            selected_tag_name: String::new(),
        }
    }
}

impl Default for Diagnositcs {
    fn default() -> Self {
        Diagnositcs {
            content: Vec::new(),
            selected_page: 0,
            selected_idx: 0,
        }
    }
}

impl Default for ListBoxQueue {
    fn default() -> Self {
        ListBoxQueue {
            content: Vec::new(),
            selected_idx: 0,
            selected_page: 0,
        }
    }
}

impl Default for ListBoxDiscover {
    fn default() -> Self {
        ListBoxDiscover {
            content: Vec::new(),
            selected_idx: 0,
            selected_page: 0,
            loadedpages: 0,
        }
    }
}

impl Default for QueuedTrack {
    fn default() -> Self {
        QueuedTrack {
            title: String::new(),
            artist: String::new(),
            audio_url: String::new(),
            album: String::new(),
            duration: 0.0,
        }
    }
}

impl State {
    pub fn switch_view(&mut self, to: CurrentView) {
        self.tags.selected_idx = 0;
        self.tags.selected_page = 0;

        self.discover.selected_idx = 0;
        self.discover.selected_page = 0;

        self.queue.selected_idx = 0;
        self.queue.selected_page = 0;

        self.queue.selected_idx = 0;
        self.queue.selected_page = 0;

        self.current_view = to
    }

    pub fn set_current_view_state(&mut self, idx: usize, page: usize) {
        match self.current_view {
            CurrentView::Tags => {
                self.tags.selected_idx = idx;
                self.tags.selected_page = page;
            }

            CurrentView::Albums => {
                self.discover.selected_idx = idx;
                self.discover.selected_page = page;
            }

            CurrentView::Queue => {
                self.queue.selected_idx = idx;
                self.queue.selected_page = page;
            }

            CurrentView::Diagnositcs => {
                self.diagnostics.selected_page = page;
                self.diagnostics.selected_idx = idx;
            }
        }
    }

    pub fn get_current_idx(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_idx,
            CurrentView::Albums => self.discover.selected_idx,
            CurrentView::Queue => self.queue.selected_idx,
            CurrentView::Diagnositcs => { self.diagnostics.selected_idx },
        }
    }

    pub fn get_current_page(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_page,
            CurrentView::Albums => self.discover.selected_page,
            CurrentView::Queue => self.queue.selected_page,
            CurrentView::Diagnositcs => { self.diagnostics.selected_page },
        }
    }

    pub fn get_len(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.content.len(),
            CurrentView::Albums => self.discover.content.len(),
            CurrentView::Queue => self.queue.content.len(),
            CurrentView::Diagnositcs => { self.diagnostics.content.len() },
        }
    }

    pub fn status_bar(&mut self, message: String, is_error: bool) {
        self.error = is_error;
        self.statusbar_text = message;
    }

    pub fn draw_line(&self, stdout: &mut std::io::Stdout, height: u16) -> Result<()> {
        let (_, rows) = size().expect("Unable to get terminal size continue work is not availble!");
        for line in 1..rows {
            &stdout
                .queue(cursor::MoveTo(height, line))?
                .queue(Print("┃"))?;
        }
        Ok(())
    }

    pub fn cleanup_tags(&mut self) {
        &self.tags.content.clear();
        self.tags.selected_idx = 0;
        self.tags.selected_page = 0;
        self.current_view = CurrentView::Tags;
    }

    pub fn cleanup_albums(&mut self) {
        &self.discover.content.clear();
        self.discover.selected_idx = 0;
        self.discover.selected_page = 0;
        self.current_view = CurrentView::Tags;
    }

    pub fn cleanup_queue(&mut self) {
        &self.queue.content.clear();
        self.queue.selected_idx = 0;
        self.queue.selected_page = 0;
        self.current_view = CurrentView::Albums;
    }

    pub fn print_diag(&mut self, message: String) {
        self.diagnostics.content.push(format!("[{:?}] {}", std::time::Instant::now(), message));
    }
}

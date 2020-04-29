use crate::model::discover;
use anyhow::Result;
use crossterm::{cursor, style::Print, terminal::size, QueueableCommand};
use std::time::{Duration, Instant};

#[derive(PartialEq, Clone)]
pub enum CurrentView {
    Albums,
    Tags,
    Queue,
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
}

#[derive(Clone)]
pub struct Playback {
    pub started_at: Instant,
    pub paused_at: Option<Instant>,
    pub pause_duration: std::time::Duration,
    pub currently_playing: QueuedTrack,
    pub is_paused: bool,
}

impl Default for Playback {
    fn default() -> Self {
        Playback {
            started_at: Instant::now(),
            paused_at: None,
            pause_duration: Duration::from_secs(0),
            currently_playing: QueuedTrack::default(),
            is_paused: true,
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub statusbar_text: String,
    pub error: bool,
    pub current_view: CurrentView,
    pub discover: ListBoxDiscover,
    pub selected_tags: Vec<String>,
    pub tags: ListBoxTag,
    pub queue: ListBoxQueue,
    pub display_tags: bool,
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
        }
    }
}

impl State {
    pub fn switch_view(&mut self, to: CurrentView) {
        self.tags.selected_idx = 0;
        self.tags.selected_page = 0;
        self.discover.selected_idx = 0;
        self.discover.selected_page = 0;
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
        }
    }

    pub fn get_current_idx(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_idx,
            CurrentView::Albums => self.discover.selected_idx,
            CurrentView::Queue => self.queue.selected_idx,
        }
    }

    pub fn get_current_page(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_page,
            CurrentView::Albums => self.discover.selected_page,
            CurrentView::Queue => self.queue.selected_page,
        }
    }

    pub fn get_len(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.content.len(),
            CurrentView::Albums => self.discover.content.len(),
            CurrentView::Queue => self.queue.content.len(),
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
                .queue(Print("|"))?;
        }
        Ok(())
    }
}

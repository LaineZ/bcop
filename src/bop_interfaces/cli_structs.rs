use crate::model::discover;
use anyhow::Result;
use crossterm::{cursor, style::Print, terminal::{Clear, size, ClearType}, QueueableCommand};
#[derive(PartialEq, Clone)]
pub enum CurrentView {
    Albums,
    Tags,
    Queue,
    Diagnositcs
}


#[derive(Clone)]
pub struct Position {
    pub selected_idx: usize,
    pub selected_page: usize,
}

impl Position {
    pub fn new() -> Self { Self { selected_idx: 0, selected_page: 0 } }
}


#[derive(Clone)]
pub struct ListBoxTag {
    pub content: Vec<String>,
    pub selected_tag_name: String,
}

#[derive(Clone)]
pub struct ListBoxDiscover {
    pub content: Vec<discover::Item>,
    pub loadedpages: i32,
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
pub struct State {
    pub statusbar_text: String,
    pub bottom_text: String,
    pub error: bool,
    pub current_view: CurrentView,
    pub discover: ListBoxDiscover,
    pub selected_tags: Vec<String>,
    pub tags: ListBoxTag,
    pub queue: Vec<QueuedTrack>,
    pub queue_pos: usize,
    pub display_tags: bool,
    pub diagnostics: Vec<String>,
    pub position: Position
}

impl Default for ListBoxTag {
    fn default() -> Self {
        ListBoxTag {
            content: Vec::new(),
            selected_tag_name: String::new(),
        }
    }
}

impl Default for ListBoxDiscover {
    fn default() -> Self {
        ListBoxDiscover {
            content: Vec::new(),
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
    pub fn switch_view(&mut self, stdout: &mut std::io::Stdout, to: CurrentView) -> Result<()> {
        self.current_view = to;
        stdout.queue(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn set_current_view_state(&mut self, stdout: &mut std::io::Stdout, idx: usize, page: usize) -> Result<()> {

        // clears screen if only switches page
        if self.get_current_page() != page {
            &stdout.queue(Clear(ClearType::All))?;
        }
        
        self.position.selected_idx = idx;
        self.position.selected_page = page;

        Ok(())
    }

    pub fn get_current_idx(&self) -> usize {
        self.position.selected_idx
    }

    pub fn get_current_page(&self) -> usize {
        self.position.selected_page
    }

    pub fn get_len(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.content.len(),
            CurrentView::Albums => self.discover.content.len(),
            CurrentView::Queue => self.queue.len(),
            CurrentView::Diagnositcs => { self.diagnostics.len() },
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
                .queue(cursor::MoveTo(height, line - 1))?
                .queue(Print("┃"))?;
        }
        Ok(())
    }

    pub fn cleanup_albums(&mut self) {
        &self.discover.content.clear();
        self.position = Position::new();
        self.current_view = CurrentView::Tags;
    }

    pub fn cleanup_queue(&mut self) {
        &self.queue.clear();
        self.position = Position::new();
        self.current_view = CurrentView::Albums;
    }

    pub fn print_diag(&mut self, message: String) {
        self.diagnostics.push(format!("[{:?}] {}", std::time::Instant::now(), message));
    }
}

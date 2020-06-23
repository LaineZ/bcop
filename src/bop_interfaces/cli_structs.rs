use crate::model::discover;

#[derive(Clone)]
pub struct QueuedTrack {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub audio_url: String,
    pub album_url: String,
    pub duration: f64,
}

#[derive(Clone)]
pub struct State {
    pub statusbar_text: String,
    pub bottom_text: String,
    pub error: bool,
    pub current_view: usize,
    pub selected_tags: Vec<String>,
    pub discover: Vec<discover::Item>,
    pub queue: Vec<QueuedTrack>,
    pub queue_pos: usize,
    pub selected_position: usize,
    pub shuffle: bool,
}

impl Default for QueuedTrack {
    fn default() -> Self {
        QueuedTrack {
            title: String::new(),
            artist: String::new(),
            audio_url: String::new(),
            album: String::new(),
            album_url: String::new(),
            duration: 0.0,
        }
    }
}

impl State {
    pub fn status_bar(&mut self, message: String, is_error: bool) {
        self.error = is_error;
        self.statusbar_text = message;
    }
}
use std::{fmt::Display, time::Duration};

use console_engine::{crossterm::terminal::size, pixel, screen::Screen, Color};
use Into;

use super::tui::MAX_FPS;

const BASE_HEADER: &str = "▶ BandcampOnlinePlayer RS | ";

/// Bottom bars interface
#[derive(Clone)]
pub struct StateBar {
    /// Header bar text
    header_text: String,
    /// Bottom bar text
    bottom_text: String,
    /// Is error occured? Set header bar background color to red
    error: bool,
    /// Screen struct
    screen: Screen,
    /// Y position of bar
    pub y: u32,
}

impl StateBar {
    /// Creates a state bars
    pub fn new() -> Self {
        let (cols, rows) =
            size().expect("Unable to get terminal size continue work is not available!");

        Self {
            header_text: BASE_HEADER.to_string(),
            bottom_text: String::from("Nothing playing..."),
            error: false,
            screen: Screen::new(cols.into(), 2),
            y: (rows as u32) - 2,
        }
    }

    /// Sets header bar error message
    pub fn error<T: AsRef<str>>(&mut self, item: T) {
        self.error = true;
        self.header_text = item.as_ref().to_string();
    }

    /// Sets header bar message
    pub fn information<T: AsRef<str>>(&mut self, item: T) {
        self.error = false;
        self.header_text = format!("{}{}", BASE_HEADER, item.as_ref());
    }

    /// Sets bottom bar info
    pub fn bottom_info<T: AsRef<str>>(&mut self, item: T) {
        self.bottom_text = item.as_ref().to_string();
    }

    /// Draws a statebar
    pub fn draw(&mut self) -> &Screen {
        self.screen.fill(pixel::pxl_bg(' ', Color::Blue));
        if !self.error {
            self.screen
                .print_fbg(0, 0, self.header_text.as_str(), Color::White, Color::Blue);
        } else {
            self.screen
                .print_fbg(0, 0, self.header_text.as_str(), Color::White, Color::Red);
        }
        self.screen
            .print_fbg(0, 1, self.bottom_text.as_str(), Color::White, Color::Blue);
        &self.screen
    }

    /// Resizes statebar
    pub fn resize(&mut self, w: u16, h: u16) {
        self.screen.clear();
        self.screen.resize(w as u32, 2);
        self.y = (h as u32) - 2;
    }
}

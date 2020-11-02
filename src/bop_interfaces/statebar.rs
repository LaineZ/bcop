use std::fmt::Display;

use Into;
use console_engine::{Color, crossterm::terminal::size, pixel, pixel::Pixel, screen::Screen};

const BASE_HEADER: &str = "▶ BandcampOnlinePlayer RS | ";

#[derive(Clone)]
pub struct StateBar {
    header_text: String,
    bottom_text: String,
    error: bool,
    screen: Screen,
    pub y: u32,
}

impl StateBar {
    pub fn new() -> Self {
        let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");

        Self {
            header_text: BASE_HEADER.to_string(),
            bottom_text: String::from("Nothing playing..."),
            error: false,
            screen: Screen::new(cols.into(), 2),
            y: (rows as u32) - 2,
        }
    }

    pub fn error<T: AsRef<str>>(&mut self, item: T) {
        self.error = true;
        self.header_text = item.as_ref().to_string();
    }

    pub fn information<T: AsRef<str>>(&mut self, item: T) {
        self.error = false;
        self.header_text = format!("{}{}", BASE_HEADER, item.as_ref());
    }

    pub fn bottom_info<T: AsRef<str>>(&mut self, item: T) {
        self.bottom_text = item.as_ref().to_string();
    }

    pub fn draw(&mut self) -> &Screen {
        self.screen.fill(pixel::pxl_bg(' ', Color::Blue));
        self.screen.print_fbg(0, 0, self.header_text.as_str(), Color::White, Color::Blue);
        self.screen.print_fbg(0, 1, self.bottom_text.as_str(), Color::White, Color::Blue);
        &self.screen
    }

    pub fn resize(&mut self, w: u16, h: u16) {
        self.screen.clear();
        self.screen.resize(w as u32, 2);
        self.y = (h as u32) - 2;
    }
}

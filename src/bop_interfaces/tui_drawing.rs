use super::tui_structs::State;
use console_fb::FrameBuffer;
use crossterm::{
    style::{Color, Colors},
    terminal::size,
};
use unicode_truncate::{Alignment, UnicodeTruncateStr};

const BASE_HEADER: &str = "▶ BandcampOnlinePlayer RS | ";

#[derive(Clone)]
pub struct ListBox {
    pub display: Vec<String>,
    pub page: usize,
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub focused: bool,
}

impl ListBox {
    pub fn new(width: u16, height: u16, x: u16, focused: bool) -> Self {
        Self {
            display: Vec::new(),
            page: 0,
            width,
            height,
            focused,
            x,
        }
    }

    pub fn add(&mut self, value: String) {
        self.display.push(value);
    }

    pub fn add_range(&mut self, value: Vec<String>) {
        self.display.extend(value);
    }

    pub fn is_empty(&mut self) -> bool {
        self.display.len() == 0
    }

    pub fn resize(&mut self, w: u16, h: u16, x: u16) {
        self.width = w;
        self.height = h;
        self.x = x;
    }

    pub fn get_page_count(&mut self) -> usize {
        self.display.chunks((self.height - 2) as usize).len()
    }

    pub fn switch_page_up(&mut self, mut stdout: &mut std::io::Stdout) {
        if self.page < self.get_page_count() {
            self.page += 1;
        }
    }

    pub fn switch_page_down(&mut self, mut stdout: &mut std::io::Stdout) {
        if self.page > 0 {
            self.page -= 1;
        }
    }

    pub fn get_selected_item(&mut self, pos: usize) -> String {
        self.display[(pos + (self.page * self.height as usize))
            .checked_sub(1)
            .unwrap_or(0)]
        .clone()
    }

    pub fn remove(&mut self, value: String) {
        self.display.retain(|x| x == &value);
    }

    pub fn clear(&mut self) {
        self.display.clear();
    }

    pub fn sel_idx_glob(&mut self, pos: usize) -> usize {
        pos + (self.page * self.height as usize)
    }

    pub fn draw(&mut self, fb: &mut FrameBuffer, state: &State) -> Result<(), anyhow::Error> {
        // drawing
        let splited_pags = self.display.chunks((self.height - 1) as usize);

        for i in 1..self.height {
            fb.set("│", self.width + 1, i);
        }

        for (i, v) in &mut splited_pags.into_iter().enumerate() {
            if i == self.page {
                for (index, page) in v.into_iter().enumerate() {
                    if index == state.selected_position && self.focused {
                        // TODO: Colors in framebuffer
                        fb.set_color(
                            self.x,
                            index as u16,
                            self.width,
                            self.height,
                            Colors::new(Color::White, Color::Reset),
                        )
                    } else {
                        fb.set_color(
                            self.x,
                            index as u16,
                            self.width,
                            self.height,
                            Colors::new(Color::Grey, Color::Reset),
                        )
                    }
                    let text = page.unicode_pad(self.width as usize - 2, Alignment::Left, true);
                    fb.set(&text.to_string(), self.x, index as u16 + 1);
                }
            }
        }

        Ok(())
    }
}

pub fn redraw(
    state: &State,
    mut fb: &mut FrameBuffer,
    listboxes: &mut std::vec::Vec<ListBox>,
) -> Result<(), anyhow::Error> {
    let (cols, _rows) =
        size().expect("Unable to get terminal size continue work is not available!");

    for (_, lists) in listboxes.iter_mut().enumerate() {
        /*
        log::info!(
            "drawn listbox: {}x{}: {} x: {}",
            lists.width,
            lists.height,
            i,
            lists.x
        );
        */
        lists.draw(&mut fb, state)?;
    }

    if !state.error {
        // TODO: Switch color to blue
    } else {
        // TODO: Switch color to red
    }

    let fixed_space = (cols as usize)
        .checked_sub(state.header_text.len())
        .unwrap_or(0)
        .checked_sub(BASE_HEADER.chars().count())
        .unwrap_or(0);

    fb.set_color(0, 0, cols, 1, Colors::new(Color::White, Color::Blue));
    fb.set(
        format!(
            "{}{}{}",
            BASE_HEADER,
            &state.header_text,
            " ".repeat(fixed_space as usize)
        ),
        0,
        0,
    );

    Ok(())
}

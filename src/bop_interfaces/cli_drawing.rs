use crossterm::style::SetBackgroundColor;
use crossterm::{cursor, QueueableCommand};
use crossterm::{
    style::{self, Print},
    terminal::size,
    ExecutableCommand,
};

use super::cli_structs::State;
use unicode_truncate::UnicodeTruncateStr;
use unicode_truncate::Alignment;

use anyhow::Result;
use style::{Color, SetForegroundColor};
const PROGRAM_NAME: &str = "▶ BandcampOnlinePlayer RS | ";

fn clear_sqr(stdout: &mut std::io::Stdout, x: u16, y: u16, w: u16, h: u16) -> Result<()> {
    /*
    for xc in x..w {
        for yc in y..h {
            &stdout.queue(cursor::MoveTo(xc, yc))?;
            &stdout.queue(Print(" "))?;
        }
    }
    */
    Ok(())
}

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

    pub fn switch_page_up(&mut self, mut stdout: &mut std::io::Stdout) -> Result<()> {
        if self.page < self.get_page_count() {
            clear_sqr(&mut stdout, self.x, 1, self.width, self.height)?;
            self.page += 1;
        }
        Ok(())
    }

    pub fn switch_page_down(&mut self, mut stdout: &mut std::io::Stdout) -> Result<()> {
        if self.page > 0 {
            clear_sqr(&mut stdout, self.x, 1, self.width, self.height)?;
            self.page -= 1;
        }
        Ok(())
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

    pub fn draw(&mut self, mut stdout: &mut std::io::Stdout, state: &State) -> Result<()> {
        // drawing
        let splited_pags = self.display.chunks((self.height - 1) as usize);

        &stdout.execute(cursor::MoveTo(self.width - 10, 1));
        //&stdout.execute(Print(format!("w: {} h: {}", self.width, self.height)));

        for i in 1..self.height {
            &stdout.execute(cursor::MoveTo(self.width + 1, i));
            &stdout.execute(Print("|"));
        }

        for (i, v) in &mut splited_pags.into_iter().enumerate() {
            if i == self.page {
                for (index, page) in v.into_iter().enumerate() {
                    if index == state.selected_position && self.focused {
                        &stdout.execute(SetBackgroundColor(Color::White))?;
                        &stdout.execute(SetForegroundColor(Color::Black))?;
                    }
                    let text = page.unicode_pad(self.width as usize - 1, Alignment::Left, true);
                    &stdout.queue(cursor::MoveTo(self.x, index as u16 + 1))?;
                    &stdout.queue(Print(text))?;
                    &stdout.execute(style::ResetColor)?;
                }
            }
        }

        Ok(())
    }
}

pub fn run_string(s: String, max_width: usize, current_offset: usize) -> String {
    if s.len() > max_width {
        s.chars().skip(current_offset).take(max_width).collect()
    } else {
        s
    }
}

pub fn redraw(
    mut stdout: &mut std::io::Stdout,
    state: &State,
    listboxes: &mut std::vec::Vec<ListBox>,
) -> Result<()> {
    let (cols, _rows) = size().expect("Unable to get terminal size continue work is not available!");
    &stdout.lock().execute(cursor::MoveTo(0, 0))?;

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
        lists.draw(&mut stdout, state)?;
    }

    if !state.error {
        &stdout.execute(SetBackgroundColor(Color::Blue))?;
    } else {
        &stdout.execute(SetBackgroundColor(Color::Red))?;
    }

    let fixed_space = (cols as usize)
        .checked_sub(state.statusbar_text.len())
        .unwrap_or(0)
        .checked_sub(PROGRAM_NAME.chars().count())
        .unwrap_or(0);

    &stdout.execute(cursor::MoveTo(0, 0))?.execute(Print(format!(
        "{}{}{}",
        PROGRAM_NAME,
        &state.statusbar_text,
        " ".repeat(fixed_space as usize)
    )));
    &stdout.execute(style::ResetColor)?;
    Ok(())
}

pub fn redraw_bottom_bar(stdout: &mut std::io::Stdout, state: &mut State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");

    let fixed_space = (cols as usize)
        .checked_sub(state.bottom_text.len())
        .unwrap_or(0);

    &stdout.execute(SetBackgroundColor(Color::DarkGrey))?;
    &stdout.execute(SetForegroundColor(Color::White))?;
    &stdout
        .execute(cursor::MoveTo(0, rows))?
        .execute(Print(format!(
            "{}{}",
            state.bottom_text,
            " ".repeat(fixed_space as usize)
        )));
    &stdout.execute(style::ResetColor)?;
    Ok(())
}

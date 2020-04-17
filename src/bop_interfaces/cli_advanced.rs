use crossterm::event::read;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::Clear;
use crossterm::cursor::DisableBlinking;
use std::time::{Duration, Instant};

use crate::bop_core;
use crate::bop_core::album_parsing;
use crate::bop_core::playback;
use crate::bop_core::playback_advanced;
use crate::model::album;
use bytes::Bytes;
use std::io::{stdout, Write, Stdout, self};
use crossterm::{execute, ExecutableCommand, terminal::{size, enable_raw_mode, ClearType}, style::{self, Colorize, Print}};
use crossterm::{event, cursor, QueueableCommand};

use anyhow::Result;
use cursor::Hide;
use style::{SetForegroundColor, Color};
use bop_core::tags;
use event::{Event::{self, Key}, KeyEvent, KeyCode};


struct State {
    statusbar_text: String,
    error: bool,
    selected_idx: usize,
    selected_tags: Vec<String>,
    selected_tag_name: String,
    selected_page: usize
}

fn redraw(stdout: &mut std::io::Stdout, tags: &Vec<String>, state: &mut State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    let pages = tags.chunks((rows - 2) as usize);

    stdout.queue(Clear(ClearType::All))?;


    for (i, v) in &mut pages.into_iter().enumerate() {
        if i == state.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.selected_idx {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    let page_str = page.to_string();
                    state.selected_tag_name = page_str;
                }

                if state.selected_tags.iter().any(|i| i==page) {
                    &stdout.execute(SetForegroundColor(Color::Red))?;
                }

                &stdout.queue(cursor::MoveTo(0,(index + 1) as u16))?.queue(Print(page))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }

    if !state.error {
        &stdout.execute(SetBackgroundColor(Color::Blue))?;
    } else {
        &stdout.execute(SetBackgroundColor(Color::Red))?;
    }

    let mut fixed_space: i32 = (cols as i32) - (state.statusbar_text.len() as i32) - 28;

    if fixed_space < 0 {
        fixed_space = 0;
    }
    &stdout.execute(cursor::MoveTo(0,0))?.execute(Print(format!("▶ BandcampOnlinePlayer RS | {}{}", &state.statusbar_text, " ".repeat((fixed_space as usize)))));
    &stdout.execute(style::ResetColor)?;
    Ok(())
}

fn switch_page_up(tags: &Vec<String>, state: &mut State) {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    state.selected_idx = 0;

    if state.selected_page < (tags.len() / (rows - 2) as usize) as usize {
        state.selected_page += 1;
    } else {
        state.error = true;
        state.statusbar_text = "You aready scrolled to end!".to_string()
    }
}

pub async fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init

    let mut stdout = stdout();

    stdout.queue(DisableBlinking)?;
    stdout.queue(Hide)?;
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(event::EnableMouseCapture)?;

    enable_raw_mode()?;

    let tags = tags::get_tags().await?;
    let mut state = State { 
        statusbar_text: "Select tags from list pressing [Space] to load tags press enter!".to_string(), 
        error: false, 
        selected_idx: 0,
        selected_page: 0,
        selected_tags: Vec::new(),
        selected_tag_name: String::new(),
    };
    redraw(&mut stdout, &tags, &mut state)?;

    loop {
        match read()? {
            Key(pressedkey) => {


                let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

                state.statusbar_text = format!("Page {}/{} Selected: {}/{} Selected tags will marked in red color", state.selected_page, (tags.len() / (rows - 2) as usize) as usize, state.selected_idx, (rows - 2) as usize);

                if pressedkey == KeyCode::Char('c').into() {
                   // TODO: Exit properly....
                   break;
                }

                if pressedkey == KeyCode::Char('d').into() {
                    state.selected_tags.clear()
                }

                if pressedkey == KeyCode::Down.into() {
                    state.selected_idx += 1;
                    if state.selected_idx > (rows - 3) as usize {
                        switch_page_up(&tags, &mut state);
                    }
                }

                if pressedkey == KeyCode::Up.into() {
                    if state.selected_idx > 0 && state.selected_idx > 0 {
                        state.selected_idx -= 1;
                    } else {
                        if state.selected_page > 0 {
                            state.selected_page -= 1;
                        }
                        state.selected_idx = (rows - 3) as usize;
                    }
                }

                if pressedkey == KeyCode::Char(' ').into() {
                    state.selected_tags.push(state.selected_tag_name.clone());
                }

                redraw(&mut stdout, &tags,  &mut state)?;
            }
            event::Event::Mouse(_) => { redraw(&mut stdout, &tags.clone(), &mut state)?; }
            event::Event::Resize(_, _) => { 
                redraw(&mut stdout, &tags.clone(), &mut state)?;
                state.selected_idx = 0;
            }
        }
    }

    Ok(())
}
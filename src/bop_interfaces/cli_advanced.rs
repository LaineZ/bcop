use crossterm::event::read;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::Clear;
use crossterm::cursor::DisableBlinking;
use std::time::{Duration, Instant};

use crate::bop_core;
use crate::bop_core::album_parsing;
use crate::bop_core::playback;
use crate::bop_core::playback_advanced;
use crate::model::{self, album};
use bytes::Bytes;
use std::io::{stdout, Write, Stdout, self};
use crossterm::{execute, ExecutableCommand, terminal::{size, enable_raw_mode, ClearType}, style::{self, Colorize, Print}};
use crossterm::{event, cursor, QueueableCommand};

use anyhow::Result;
use cursor::Hide;
use style::{SetForegroundColor, Color};
use bop_core::tags;
use event::{Event::{self, Key}, KeyEvent, KeyCode};
use model::discover;
#[derive(PartialEq)]
enum CurrentView {
    Albums,
    Tags
}


struct State {
    statusbar_text: String,
    error: bool,
    selected_idx: usize,
    selected_tags: Vec<String>,
    selected_tag_name: String,
    selected_page: usize,
    discover: Vec<discover::Item>,
    current_view: CurrentView,
}

impl State {
    fn switch_view(&mut self, to: CurrentView) {
        self.selected_idx = 0;
        self.selected_page = 0;
        self.current_view = to
    }
}

fn redraw(stdout: &mut std::io::Stdout, tags: &Vec<String>, state: &mut State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    let lineheight = tags.iter().max_by_key(|p| p.len()).unwrap().len() as u16;
    let pages = tags.chunks((rows - 2) as usize);
    let album_pages = state.discover.chunks((rows - 2) as usize); 

    stdout.queue(Clear(ClearType::All))?;

    for (i, v) in &mut pages.into_iter().enumerate() {
        if i == state.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.selected_idx && state.current_view == CurrentView::Tags {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    let page_str = page.to_string();
                    state.selected_tag_name = page_str;
                }

                if state.selected_tags.iter().any(|i| i==page) {
                    &stdout.execute(SetForegroundColor(Color::Red))?;
                }
                if state.current_view != CurrentView::Tags { &stdout.execute(SetForegroundColor(Color::Grey))?; }

                &stdout.queue(cursor::MoveTo(0,(index + 1) as u16))?.queue(Print(page))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }

    for (i, v) in &mut album_pages.into_iter().enumerate() {
        if i == state.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.selected_idx {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    //state.selected_tag_name = page_str;
                }

                if state.current_view != CurrentView::Albums { &stdout.execute(SetBackgroundColor(Color::Grey))?; }
                
                let formatting = format!("{} by {}", page.clone().title, page.clone().artist);
                &stdout.queue(cursor::MoveTo(lineheight + 1,(index + 1) as u16))?.queue(Print(formatting))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }


    for line in 1..rows {
        &stdout.queue(cursor::MoveTo(lineheight, line))?.queue(Print("|"))?;
    }

    if !state.error {
        &stdout.execute(SetBackgroundColor(Color::Blue))?;
    } else {
        &stdout.execute(SetBackgroundColor(Color::Red))?;
    }

    let mut fixed_space: i32 = (cols as i32) - (state.statusbar_text.len() as i32) - 28;

    // test usize oveflow, lol
    if fixed_space < 0 {
        fixed_space = 0;
    }

    &stdout.execute(cursor::MoveTo(0,0))?.execute(Print(format!("▶ BandcampOnlinePlayer RS | {}{}", &state.statusbar_text, " ".repeat(fixed_space as usize))));
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
        discover: Vec::new(),
        current_view: CurrentView::Tags,
    };
    redraw(&mut stdout, &tags, &mut state)?;

    loop {
        match read()? {
            Key(pressedkey) => {


                let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

                state.statusbar_text = format!("Page {}/{} Selected: {}/{}", state.selected_page, (tags.len() / (rows - 2) as usize) as usize, state.selected_idx, (rows - 2) as usize);

                if pressedkey == KeyCode::Char('c').into() {
                   // TODO: Exit properly....
                   break;
                }

                if pressedkey == KeyCode::Enter.into() {
                    state.switch_view(CurrentView::Albums);
                    while state.discover.len() < (rows - 2) as usize {
                        state.statusbar_text = format!("Discovering");
                        let discover = album_parsing::get_tag_data(state.selected_tags.clone()[0].clone(), 1).await?.items;
                        state.discover.extend(discover);
                    }
                    state.statusbar_text = format!("Done!");
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
                    // TODO: if aready added - clear
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
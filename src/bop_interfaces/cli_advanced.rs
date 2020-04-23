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

struct ListBoxTag {
    content: Vec<String>,
    selected_idx: usize,
    selected_page: usize,
    selected_tag_name: String,
}

struct ListBoxDiscover {
    content: Vec<discover::Item>,
    selected_idx: usize,
    selected_page: usize,
    loadedpages: i32,
}

struct State {
    statusbar_text: String,
    error: bool,
    current_view: CurrentView,
    discover: ListBoxDiscover,
    selected_tags: Vec<String>,
    tags: ListBoxTag,
}

impl Default for ListBoxTag {
    fn default() -> ListBoxTag {
        ListBoxTag {
            content: Vec::new(),
            selected_idx: 0,
            selected_page: 0,
            selected_tag_name: String::new(),
        }
    }
}

impl Default for ListBoxDiscover {
    fn default() -> ListBoxDiscover {
        ListBoxDiscover {
            content: Vec::new(),
            selected_idx: 0,
            selected_page: 0,
            loadedpages: 0,
        }
    }
}


impl State {
    fn switch_view(&mut self, to: CurrentView) {
        self.tags.selected_idx = 0;
        self.tags.selected_page = 0;
        self.discover.selected_idx = 0;
        self.discover.selected_page = 0;
        self.current_view = to
    }
    
    fn set_current_view_state(&mut self, idx: usize, page: usize) {
        match self.current_view {
            CurrentView::Tags => {
                self.tags.selected_idx = idx;
                self.tags.selected_page = page;
            }

            CurrentView::Albums => {
                self.discover.selected_idx = idx;
                self.discover.selected_page = page;
            }
        }
    }

    fn get_current_idx(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_idx,
            CurrentView::Albums => self.discover.selected_idx,
        }
    }

    fn get_current_page(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.selected_page,
            CurrentView::Albums => self.discover.selected_page,
        }
    }

    fn get_len(&self) -> usize {
        match self.current_view {
            CurrentView::Tags => self.tags.content.len(),
            CurrentView::Albums => self.discover.content.len(),
        }
    }

    fn status_bar(&mut self, message: String, is_error: bool) {
        self.error = is_error;
        self.statusbar_text = message;
    }
}

fn redraw(stdout: &mut std::io::Stdout, state: &mut State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    let lineheight = state.tags.content.iter().max_by_key(|p| p.len()).unwrap().len() as u16;

    let lineheight_album = state.discover.content.iter().max_by_key(|p| format!("{} by {}", p.title, p.artist).len());
    let mut lineheight_album_int: u16 = lineheight;
    match lineheight_album {
        Some(value) => lineheight_album_int += format!("{} by {}", value.title, value.artist).len() as u16,
        None => lineheight_album_int += 20,
    }

    let pages = state.tags.content.chunks((rows - 2) as usize);
    let album_pages = state.discover.content.chunks((rows - 2) as usize); 

    stdout.queue(Clear(ClearType::All))?;

    for (i, v) in &mut pages.into_iter().enumerate() {
        if i == state.tags.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.tags.selected_idx && state.current_view == CurrentView::Tags {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    let page_str = page.to_string();
                    state.tags.selected_tag_name = page_str;
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
        if i == state.discover.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.discover.selected_idx {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    //state.selected_tag_name = page_str;
                }


                if state.current_view != CurrentView::Albums { &stdout.execute(SetForegroundColor(Color::Grey))?; }
                
                let formatting = format!("{} by {}", page.clone().title, page.clone().artist);
                &stdout.queue(cursor::MoveTo(lineheight + 1,(index + 1) as u16))?.queue(Print(formatting))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }

    // drawing lines
    for line in 1..rows {
        &stdout.queue(cursor::MoveTo(lineheight, line))?.queue(Print("|"))?;
    }

    for line in 1..rows {
        &stdout.queue(cursor::MoveTo(lineheight_album_int, line))?.queue(Print("|"))?;
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

async fn switch_page_up(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let idx = state.get_current_idx();
    let page = state.get_current_page();

    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    if page < (state.get_len() / (rows - 2) as usize) as usize {
        state.set_current_view_state(idx, page + 1);
    } else {
        state.status_bar("You aready scrolled to end!".to_string(), true);
    }

    // stream loading
    if state.current_view == CurrentView::Albums {
        state.status_bar("Loading next page...".to_string(), false);
        state.discover.loadedpages += 1;
        let discover = album_parsing::get_tag_data(state.selected_tags.clone()[0].clone(), state.discover.loadedpages).await?.items;
        state.discover.content.extend(discover);
    }
    Ok(())
}

pub async fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init

    let mut stdout = stdout();

    println!("Loading tags from bandcamp.com");
    let tags = tags::get_tags().await?;
    println!("Loading gui...");

    stdout.queue(DisableBlinking)?;
    stdout.queue(Hide)?;
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(event::EnableMouseCapture)?;

    enable_raw_mode()?;

    let mut state = State { 
        statusbar_text: "[Space]: Select Tags [Enter]: Load tag albums".to_string(), 
        error: false, 
        current_view: CurrentView::Tags,
        tags: ListBoxTag::default(),
        selected_tags: Vec::new(),
        discover: ListBoxDiscover::default(),
    };
    state.tags.content = tags;
    redraw(&mut stdout, &mut state)?;

    loop {
        match read()? {
            Key(pressedkey) => {


                let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

                if pressedkey == KeyCode::Char('c').into() {
                   // TODO: Exit properly....
                   break;
                }

                if pressedkey == KeyCode::Enter.into() {
                    if state.current_view == CurrentView::Tags {
                        state.statusbar_text = format!("Discovering");
                        state.switch_view(CurrentView::Albums);
                        while state.discover.content.len() < (rows - 2) as usize {
                            state.discover.loadedpages += 1;
                            let discover = album_parsing::get_tag_data(state.selected_tags.clone()[0].clone(), state.discover.loadedpages).await?.items;
                            state.discover.content.extend(discover);
                        }
                        state.statusbar_text = format!("Done!");
                    } else {
                        state.status_bar("Sorry currently playback does not work".to_string(), true);
                    }
                 }

                if pressedkey == KeyCode::Char('d').into() {
                    &state.selected_tags.clear();
                }

                if pressedkey == KeyCode::Tab.into() {
                    if state.current_view == CurrentView::Albums {
                        &state.switch_view(CurrentView::Tags);
                    } else {
                        &state.switch_view(CurrentView::Albums);
                    }
                }

                if pressedkey == KeyCode::Down.into() {
                    state.set_current_view_state(state.get_current_idx() + 1, state.get_current_page());
                    if state.get_current_idx() > (rows - 3) as usize {
                        state.set_current_view_state(0, state.get_current_page());
                        switch_page_up(&mut state).await?;
                    }
                }

                if pressedkey == KeyCode::Up.into() {
                    if state.get_current_idx() > 0 {
                        state.set_current_view_state(state.get_current_idx() - 1, state.get_current_page());
                    } else {
                        if state.get_current_page() > 0 {
                            state.set_current_view_state(state.get_current_idx(), state.get_current_page() - 1);
                        }
                        state.set_current_view_state((rows - 3) as usize, state.get_current_page());
                    }
                }

                if pressedkey == KeyCode::Char(' ').into() {
                    // TODO: if aready added - clear
                    state.selected_tags.push(state.tags.selected_tag_name.clone());
                }

                redraw(&mut stdout, &mut state)?;
            }
            event::Event::Mouse(_) => { redraw(&mut stdout, &mut state)?; }
            event::Event::Resize(_, _) => { 
                redraw(&mut stdout, &mut state)?;
                state.set_current_view_state(0, state.get_current_page());
            }
        }
    }

    Ok(())
}
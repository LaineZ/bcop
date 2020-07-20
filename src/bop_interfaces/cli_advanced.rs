use crossterm::cursor::DisableBlinking;
use crossterm::event::read;
use crossterm::terminal::Clear;
use std::{
    io::stdout,
    sync::{Arc, Mutex},
    time::Duration,
};
use webbrowser;

use super::cli_drawing;
use crate::bc_core;
use crate::bc_core::album_parsing;
use rand::prelude::*;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use crossterm::{cursor, event, QueueableCommand};
use crossterm::{
    style,
    terminal::{disable_raw_mode, enable_raw_mode, size, ClearType},
    ExecutableCommand,
};

use super::{
    cli_drawing::redraw,
    cli_structs::{QueuedTrack, State, Queue}, cli_optimized::FrameBuffer,
};

use anyhow::Result;

use bc_core::playback::{FormatTime, Player};
use cli_drawing::ListBox;
use cursor::{EnableBlinking, Hide, Show};
use event::{poll, Event::Key, KeyCode};
use style::Colorize;

const COLS_COUNT: u16 = 2;

const LIST_BOX_TAGS: usize = 0;
const LIST_BOX_DISCOVER: usize = 1;
const LIST_BOX_QUEUE: usize = 2;

macro_rules! block {
    ($xs:block) => {
        loop {
            let _ = $xs;
            break;
        }
    };
}

pub fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init
    let stdout = Arc::new(Mutex::new(stdout()));

    let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");

    log::info!("Detected terminal size {}x{}", cols, rows);

    let tags = include_str!("tags.list")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();
    println!("Loading gui...");

    {
        let stdout_clone = stdout.clone();
        let mut stdout = stdout_clone.lock().unwrap();
        stdout.queue(DisableBlinking)?;
        stdout.queue(Hide)?;
        stdout.queue(Clear(ClearType::All))?;
        //stdout.queue(event::EnableMouseCapture)?;
    }

    enable_raw_mode()?;

    let mut state = State {
        statusbar_text: "[Space]: Select Tags [Enter]: Load tag albums".to_string(),
        bottom_text: "Nothing...".to_string(),
        error: false,
        current_view: LIST_BOX_TAGS,
        selected_tags: Vec::new(),
        discover: Vec::new(),
        selected_position: 0,
    };

    let mut listboxes = Vec::new();

    // init listboxes (PLEASE KEEP ORDER WITH CONSTANTS)
    // tags
    listboxes.push(ListBox::new(15, rows - 1, 0, true));
    // discover
    listboxes.push(ListBox::new(
        cols / COLS_COUNT,
        rows - 1,
        listboxes[LIST_BOX_TAGS].width + 2,
        false,
    ));
    // queue
    listboxes.push(ListBox::new(
        cols / COLS_COUNT,
        rows - 1,
        (listboxes[LIST_BOX_DISCOVER].width) + 2,
        false,
    ));

    // push default tags

    listboxes[LIST_BOX_TAGS].add_range(tags);
    redraw(&mut stdout.lock().unwrap(), &state, &mut listboxes)?;

    let mut loadedpages = 1;
    let player = Arc::new(Mutex::new(Player::new()));

        let playern = player.clone();
        let mut queue_manager = Queue::new(Box::new(move |track| {
            //state.bottom_text = format!("Loading track: {} - {}", track.artist, track.title);
            playern.lock().unwrap().switch_track(track.audio_url);
        }));

    let mut running_text_offset: usize = 0;

    loop {
        while !poll(Duration::from_millis(250))? {
            let ctrl_text = format!(
                "volume: {}% shuffle: {}",
                player.lock().unwrap().get_volume(),
                queue_manager.shuffle
            );

            let (cols, _rows) =
                size().expect("Unable to get terminal size continue work is not available!");

            if let Some(time) = player.lock().unwrap().get_time() {
                match queue_manager.get_current_track() {
                    Some(track) => {
                        if !listboxes[LIST_BOX_QUEUE].is_empty() {
                            let mins = track.duration / 60.0;
                            let secs = track.duration % 60.0;
        
                            let track_title_base = format!(
                                "{} - {} ",
                                track.artist, track.title
                            );
        
                            if track_title_base.len() >= running_text_offset {
                                running_text_offset += 1;
                            } else {
                                running_text_offset = 0;
                            }
        
                            let mut split_size: usize = 0;
        
                            if track_title_base.len() > (cols / 2) as usize {
                                split_size = (cols / 2) as usize;
                            }
        
                            let track_title = cli_drawing::run_string(
                                track_title_base,
                                (cols / 2) as usize,
                                running_text_offset,
                            );
                            let whitespace =
                                " ".repeat(split_size.checked_sub(track_title.len()).unwrap_or(0));
        
                            //log::info!("{}", track_title);
                            state.bottom_text = format!(
                                "\r{}/{:02}:{:02} {} {} pos: {} {}",
                                FormatTime(time),
                                mins as u32,
                                secs as u32,
                                track_title,
                                whitespace,
                                queue_manager.queue_pos,
                                ctrl_text
                            );
        
                            if track.duration - time.as_secs_f64() < 1.0
                            {
                                if !queue_manager.shuffle {
                                    queue_manager.next();
                                } else {
                                    // TODO: Shuffle
                                }
                            }
                        }
                    }
                    None => {
                        state.bottom_text = format!("⯀ {}", ctrl_text);
                    }
                }
            } else {
                state.bottom_text = format!("⯀ {}", ctrl_text);
            }
            cli_drawing::redraw_bottom_bar(&mut stdout.lock().unwrap(), &mut state)?;
        }

        let event = read()?;

        if state.error {
            state.status_bar(String::new(), false);
            redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
        }

        match event {
            Key(pressedkey) => {
                let (_cols, rows) =
                    size().expect("Unable to get terminal size continue work is not available!");

                if pressedkey == KeyCode::Esc.into() {
                    disable_raw_mode()?;
                    {
                        let mut stdout = stdout.lock().unwrap();

                        stdout.queue(EnableBlinking)?;
                        stdout.queue(Show)?;
                        stdout.queue(Clear(ClearType::All))?;
                    }
                    break;
                }

                if pressedkey == KeyCode::Enter.into() {
                    if state.current_view == LIST_BOX_TAGS {
                        if state.selected_tags.len() > 0 {
                            while state.discover.len() <= (rows - 2) as usize {
                                loadedpages += 1;
                                log::info!("Loading discover: {}", loadedpages);
                                let discover = album_parsing::get_tag_data(
                                    state.selected_tags.clone(),
                                    loadedpages,
                                )?
                                .items;
                                state.discover.extend(discover);
                            }

                            for item in state.discover.iter() {
                                listboxes[LIST_BOX_DISCOVER]
                                    .add(format!("{} - {}", item.artist, item.title));
                            }
                        }
                    }
                    if state.current_view == LIST_BOX_DISCOVER {
                        queue_manager.add_album_in_queue(state.discover[state.selected_position].artist.clone(), state.discover[state.selected_position].tralbum_url.as_str());
                    }
                    if state.current_view == LIST_BOX_QUEUE {
                        queue_manager.set(state.selected_position);
                    }
                }

                if pressedkey == KeyCode::Delete.into() {
                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;

                    match state.current_view {
                        LIST_BOX_TAGS => {
                            &state.selected_tags.clear();
                        }

                        LIST_BOX_QUEUE => {
                            player.lock().unwrap().pause();
                            player.lock().unwrap().stop();
                            listboxes[LIST_BOX_QUEUE].clear();
                        }

                        LIST_BOX_DISCOVER => {
                            listboxes[state.current_view].clear();
                            state.discover.clear();
                        }

                        _ => {
                            listboxes[state.current_view].clear();
                        }
                    }
                    if state.current_view == LIST_BOX_QUEUE {
                        // stop playback
                        player.lock().unwrap().pause();
                        player.lock().unwrap().stop();
                    }
                }

                if pressedkey == KeyCode::Char('o').into() {
                    match queue_manager.get_current_track() {
                        Some(track) => {
                            webbrowser::open(track.album_url.as_str())?;
                        }

                        None => {
                            state.status_bar(String::from("Queue list is empty!"), true);
                        }
                    }
                }

                if pressedkey == KeyCode::Char('q').into() {
                    queue_manager.shuffle = !queue_manager.shuffle;
                }

                if pressedkey == KeyCode::Char('e').into() {
                    player.lock().unwrap().stop();
                }

                if pressedkey == KeyCode::Char('c').into() {
                    let mut ctx: ClipboardContext = ClipboardProvider::new()?;

                    let content = ctx.get_contents();
                    match content {
                        Ok(clipboard) => {
                            block!({
                                if !clipboard.contains("bandcamp.com") {
                                    state.status_bar(
                                        String::from("Clipboard contains non-bandcamp URL!"),
                                        true,
                                    );
                                    break;
                                }

                                queue_manager.add_album_in_queue(clipboard.clone(), clipboard.as_str())
                            });
                        }
                        Err(_) => {
                            state
                                .status_bar(String::from("Clipboard contains invalid data!"), true);
                        }
                    }
                }

                if pressedkey == KeyCode::Char('q').into() {
                    state.current_view = LIST_BOX_QUEUE;
                }

                if pressedkey == KeyCode::Tab.into() {
                    if listboxes.len() - 1 == state.current_view {
                        state.current_view = 0;
                    } else {
                        state.current_view += 1;
                    }

                    listboxes[LIST_BOX_DISCOVER].focused = false;
                    listboxes[LIST_BOX_TAGS].focused = false;
                    listboxes[LIST_BOX_QUEUE].focused = false;

                    listboxes[state.current_view].focused = true;
                }

                if pressedkey == KeyCode::Up.into() {
                    // TODO: up scrolling
                    if state.selected_position > 0 {
                        state.selected_position -= 1;
                    } else {
                        listboxes[state.current_view]
                            .switch_page_down(&mut stdout.lock().unwrap())?;
                        state.selected_position = listboxes[state.current_view].height as usize - 2;
                    }
                }

                if pressedkey == KeyCode::Down.into() {
                    // TODO: Sroll down
                    if state.selected_position < listboxes[state.current_view].height as usize - 2 {
                        state.selected_position += 1;
                    } else {
                        listboxes[state.current_view]
                            .switch_page_up(&mut stdout.lock().unwrap())?;
                        state.selected_position = 0;
                        if loadedpages > 0 && state.current_view == LIST_BOX_DISCOVER {
                            loadedpages += 1;

                            log::info!("Loading discover: {}", loadedpages);
                            let discover = album_parsing::get_tag_data(
                                state.selected_tags.clone(),
                                loadedpages,
                            )?
                            .items;
                            state.discover.extend(discover.clone());

                            for item in discover.iter() {
                                listboxes[LIST_BOX_DISCOVER]
                                    .add(format!("{} - {}", item.artist, item.title));
                            }
                        }
                    }
                }

                if pressedkey == KeyCode::Left.into() {
                    state.bottom_text = "Tracking back by 5 seconds... Please wait...".to_string();
                    player.lock().unwrap().seek_backward(Duration::from_secs(5));
                }

                if pressedkey == KeyCode::Char('w').into() {
                    if player.lock().unwrap().get_volume() < 100 {
                        player.lock().unwrap().increase_volume(1);
                    }
                }

                if pressedkey == KeyCode::Char('s').into() {
                    player.lock().unwrap().decrease_volume(1);
                }

                if pressedkey == KeyCode::Right.into() {
                    state.bottom_text =
                        "Tracking forward by 5 seconds... Please wait...".to_string();
                    player.lock().unwrap().seek_forward(Duration::from_secs(5));
                }

                if pressedkey == KeyCode::PageUp.into() {
                    listboxes[state.current_view].switch_page_down(&mut stdout.lock().unwrap())?;
                }

                if pressedkey == KeyCode::PageDown.into() {
                    listboxes[state.current_view].switch_page_up(&mut stdout.lock().unwrap())?;
                }

                if pressedkey == KeyCode::Char(' ').into() {
                    // TODO: if aready added - clear
                    if state.current_view == LIST_BOX_TAGS {
                        state.selected_tags.push(
                            listboxes[LIST_BOX_TAGS].get_selected_item(state.selected_position),
                        );
                    } else {
                        // TODO: Play pause goes here
                        player.lock().unwrap().set_paused(!player.lock().unwrap().is_paused());
                    }
                }

                redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
            }
            event::Event::Mouse(_) => {
                redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
            }
            event::Event::Resize(w, h) => {
                if w > 50 && h > 10 {
                    listboxes[LIST_BOX_DISCOVER].clone().resize(15, rows - 1, 0);
                    listboxes[LIST_BOX_DISCOVER].clone().resize(
                        cols / COLS_COUNT,
                        rows - 1,
                        listboxes[LIST_BOX_TAGS].width + 2,
                    );
                    listboxes[LIST_BOX_QUEUE].clone().resize(
                        cols / COLS_COUNT,
                        rows - 1,
                        listboxes[LIST_BOX_DISCOVER].width + 2,
                    );

                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;
                    redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
                } else {
                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;
                    &stdout.lock().unwrap().execute(cursor::MoveTo(0, 0))?;
                    &stdout
                        .lock()
                        .unwrap()
                        .execute(style::PrintStyledContent("terminal is too small".red()))?;
                }
            }
        }
    }
    Ok(())
}

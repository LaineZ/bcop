use crossterm::cursor::DisableBlinking;
use crossterm::event::read;
use crossterm::terminal::Clear;
use std::{io::stdout, sync::{Mutex, Arc}, time::Duration};
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
    cli_structs::{QueuedTrack, State, },
};

use anyhow::Result;

use bc_core::{
    playback::{FormatTime, Player},
};
use cursor::{EnableBlinking, Hide, Show};
use event::{poll, Event::Key, KeyCode};
use style::Colorize;
use cli_drawing::ListBox;

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

    let (cols, rows) =
    size().expect("Unable to get terminal size continue work is not available!"); 

    log::info!("Detected terminal size {}x{}", cols, rows);

    let tags = include_str!("tags.list").split("\n").map(|s| s.trim().to_string()).collect();
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
        shuffle: false,
        current_view: LIST_BOX_TAGS,
        selected_tags: Vec::new(),
        discover: Vec::new(),
        queue: Vec::new(),
        selected_position: 0,
        queue_pos: 0,
    };

    let mut listboxes = Vec::new();

    // init listboxes (PLEASE KEEP ORDER WITH CONSTANTS)
    // tags
    listboxes.push(ListBox::new(15, rows - 1,0, true));
    // discover
    listboxes.push(ListBox::new(cols / COLS_COUNT, rows - 1, listboxes[LIST_BOX_TAGS].width + 2, false));
    // queue
    listboxes.push(ListBox::new(cols / COLS_COUNT, rows - 1,  (listboxes[LIST_BOX_DISCOVER].width) + 2, false));

    // push default tags 

    listboxes[LIST_BOX_TAGS].add_range(tags);
    redraw(&mut stdout.lock().unwrap(), &state, &mut listboxes)?;

    let mut player = Player::new();

    let mut running_text_offset: usize = 0;

    loop {
        // 60 FPS rendering, lol
        while !poll(Duration::from_millis(250))? {
            let ctrl_text = format!(
                "volume: {}% shuffle: {}",
                player.get_volume(),
                state.shuffle
            );

            let (cols, _rows) =
            size().expect("Unable to get terminal size continue work is not available!");    

            if let Some(time) = player.get_time() {
                if !listboxes[LIST_BOX_QUEUE].is_empty() {
                    let mins = state.queue[state.queue_pos].duration / 60.0;
                    let secs = state.queue[state.queue_pos].duration % 60.0;

                    let track_title_base = format!("{} - {} ", state.queue[state.queue_pos].artist,
                    state.queue[state.queue_pos].title);

                    if track_title_base.len() >= running_text_offset {
                        running_text_offset += 1;
                    } else {
                        running_text_offset = 0;
                    }

                    let mut split_size: usize = 0;
                    
                    if track_title_base.len() > (cols / 2) as usize
                    {
                        split_size = (cols / 2) as usize;
                    }

                    let track_title = cli_drawing::run_string(track_title_base, (cols / 2) as usize, running_text_offset);
                    let whitespace = " ".repeat(split_size.checked_sub(track_title.len()).unwrap_or(0));

                    //log::info!("{}", track_title); 
                    state.bottom_text = format!(
                        "\r{}/{:02}:{:02} {} {} pos: {} {}",
                        FormatTime(time),
                        mins as u32,
                        secs as u32,
                        track_title,
                        whitespace,
                        state.queue_pos,
                        ctrl_text
                    );

                    if (state.queue[state.queue_pos].duration - time.as_secs_f64()) < 1.0
                        && state.queue.len() - 1 > state.queue_pos
                    {
                        if !state.shuffle {
                            state.queue_pos += 1;
                        } else {
                            let mut rng = rand::thread_rng();
                            state.queue_pos = rng.gen_range(0, state.queue.len());
                        }

                        state.bottom_text = format!(
                            "Loading track: {} - {}",
                            state.queue[state.queue_pos].artist,
                            state.queue[state.queue_pos].title
                        );
                        player.switch_track(state.queue[state.queue_pos].audio_url.clone());
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
                            let mut loadedpages = 1;
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
                                listboxes[LIST_BOX_DISCOVER].add(format!("{} - {}", item.artist, item.title));
                            }
                        }
                    }
                    if state.current_view == LIST_BOX_DISCOVER {
                        let is_album = album_parsing::get_album(
                            state.discover[listboxes[LIST_BOX_DISCOVER].sel_idx_glob(state.selected_position)]
                                .tralbum_url
                                .as_str(),
                        );

                        match is_album {
                            Some(album) => {
                                let album_url = album
                                    .clone()
                                    .url
                                    .unwrap_or("https://ipotekin.bandcamp.com/".to_string());

                                for album_track in album.trackinfo.unwrap() {
                                    match album_track.file.clone() {
                                        Some(album_url) => {
                                            let pushed_track = QueuedTrack {
                                                album: album
                                                    .current
                                                    .clone()
                                                    .title
                                                    .unwrap_or("Unknown album".to_string()),
                                                artist: state.discover
                                                    [listboxes[LIST_BOX_DISCOVER].sel_idx_glob(state.selected_position)]
                                                    .clone()
                                                    .artist,
                                                title: album_track
                                                    .title
                                                    .unwrap_or("Unknown track title".to_string()),
                                                // TODO: switch to normal error-handling and not this garbage that panic...
                                                audio_url: album_track.file.unwrap().mp3128,
                                                album_url: album_url.mp3128,
                                                duration: album_track.duration.unwrap_or(0.0),
                                            };
                                            state.queue.push(pushed_track.clone());
                                            listboxes[LIST_BOX_QUEUE].add(format!("{} - {}", pushed_track.artist, pushed_track.title));
                                        }
                                        None => {}
                                    }
                                }
                            }
                            _ => state.status_bar(
                                format!(
                                    "Something went wrong while loading {}",
                                    state.discover[listboxes[LIST_BOX_DISCOVER].sel_idx_glob(state.selected_position)].title
                                ),
                                true,
                            ),
                        }
                    }
                    if state.current_view == LIST_BOX_QUEUE {
                        player.switch_track(
                            state.queue[state.selected_position]
                                .audio_url
                                .clone(),
                        );
                        state.queue_pos = state.selected_position;
                    }
                }

                if pressedkey == KeyCode::Delete.into() {
                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;

                    match state.current_view {
                        LIST_BOX_TAGS => {
                            &state.selected_tags.clear();
                        }

                        LIST_BOX_QUEUE => {
                            player.pause();
                            player.stop();
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
                        player.pause();
                        player.stop();
                    }
                }

                if pressedkey == KeyCode::Char('o').into() {
                    webbrowser::open(&state.queue[state.queue_pos].album_url)?;
                }

                if pressedkey == KeyCode::Char('q').into() {
                    state.shuffle = !state.shuffle;
                }

                if pressedkey == KeyCode::Char('e').into() {
                    player.stop();
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

                                let is_album = album_parsing::get_album(clipboard.as_str());

                                match is_album {
                                Some(album) => {
                                    let album_url = album
                                        .clone()
                                        .url
                                        .unwrap_or("https://ipotekin.bandcamp.com/".to_string());
                                    for album_track in album.trackinfo.unwrap() {
                                        state.queue.push(QueuedTrack {
                                            album: album
                                                .current
                                                .clone()
                                                .title
                                                .unwrap_or("Unknown album".to_string()),
                                            artist: album.current.clone().artist.unwrap_or("Unknown artist".to_string()),
                                            title: album_track
                                                .title
                                                .unwrap_or("Unknown track title".to_string()),
                                            // TODO: switch to normal error-handling and not this garbage that panic...
                                            audio_url: album_track.file.ok_or_else(|| anyhow::anyhow!("Failed to get mp3 link!"))?.mp3128,
                                            album_url: album_url.clone(),
                                            duration: album_track.duration.unwrap_or(0.0),
                                        });
                                    }
                                }
                                _ => state.status_bar(
                                    format!(
                                        "Something went wrong while loading album from clipboard: {}...",
                                        clipboard.chars().take(20).collect::<String>()
                                    ),
                                    true,
                                ),
                            }
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
                        listboxes[state.current_view].switch_page_down(&mut stdout.lock().unwrap())?;
                        state.selected_position = listboxes[state.current_view].height as usize - 2;
                    }
                }

                if pressedkey == KeyCode::Down.into() {
                    // TODO: Sroll down
                    if state.selected_position < listboxes[state.current_view].height as usize - 2 {
                        state.selected_position += 1;
                    } else {
                        listboxes[state.current_view].switch_page_up(&mut stdout.lock().unwrap())?;
                        state.selected_position = 0;
                    }
                }

                if pressedkey == KeyCode::Left.into() {
                    state.bottom_text = "Tracking back by 5 seconds... Please wait...".to_string();
                    player.seek_backward(Duration::from_secs(5));
                }

                if pressedkey == KeyCode::Char('w').into() {
                    if player.get_volume() < 100 {
                        player.increase_volume(1);
                    }
                }

                if pressedkey == KeyCode::Char('s').into() {
                    player.decrease_volume(1);
                }

                if pressedkey == KeyCode::Right.into() {
                    state.bottom_text =
                        "Tracking forward by 5 seconds... Please wait...".to_string();
                    player.seek_forward(Duration::from_secs(5));
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
                        state.selected_tags.push(listboxes[LIST_BOX_TAGS].get_selected_item(state.selected_position));
                    } else {
                        // TODO: Play pause goes here
                        player.set_paused(!player.is_paused());
                    }
                }

                redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
            }
            event::Event::Mouse(_) => {
                redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
            }
            event::Event::Resize(w, h) => {
                if w > 50 && h > 10 {
                    listboxes[LIST_BOX_DISCOVER].clone().resize(15, rows - 1,0);
                    listboxes[LIST_BOX_DISCOVER].clone().resize(cols / COLS_COUNT, rows - 1, listboxes[LIST_BOX_TAGS].width + 2);
                    listboxes[LIST_BOX_QUEUE].clone().resize(cols / COLS_COUNT, rows - 1,  listboxes[LIST_BOX_DISCOVER].width + 2);

                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;
                    redraw(&mut stdout.lock().unwrap(), &mut state, &mut listboxes)?;
                } else {
                    &stdout.lock().unwrap().execute(Clear(ClearType::All))?;
                    &stdout.lock().unwrap().execute(cursor::MoveTo(0, 0))?;
                    &stdout
                        .lock().unwrap()
                        .execute(style::PrintStyledContent("terminal is too small".red()))?;
                }
            }
        }
    }
    Ok(())
}

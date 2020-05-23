use crossterm::cursor::DisableBlinking;
use crossterm::event::read;
use crossterm::terminal::Clear;
use std::{io::stdout, sync::Arc, time::Duration};
use webbrowser;

use super::cli_drawing;
use crate::bc_core;
use crate::bc_core::album_parsing;

use crossterm::{cursor, event, QueueableCommand};
use crossterm::{
    style,
    terminal::{disable_raw_mode, enable_raw_mode, size, ClearType},
    ExecutableCommand,
};

use parking_lot::FairMutex;

use super::{
    cli_drawing::redraw,
    cli_structs::{
        CurrentView, ListBoxDiagnositcs, ListBoxDiscover, ListBoxQueue, ListBoxTag, QueuedTrack,
        State,
    },
};

use anyhow::Result;

use bc_core::{
    playback::{FormatTime, Player},
    tags,
};
use cursor::{EnableBlinking, Hide, Show};
use event::{poll, Event::Key, KeyCode};
use style::Colorize;

fn switch_page_up(mut stdout: &mut std::io::Stdout, state: &mut State) -> Result<()> {
    let idx = state.selected_position;
    let page = state.get_current_page();

    let (_cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    if page < (state.get_len() / (rows - 2) as usize) as usize {
        state.set_current_view_state(&mut stdout, idx, page + 1)?;
    } else {
        state.status_bar("You aready scrolled to end!".to_string(), true);
    }

    // stream loading
    if state.current_view == CurrentView::Albums {
        state.status_bar("Loading next page...".to_string(), false);
        state.discover.loadedpages += 1;
        let discover =
            album_parsing::get_tag_data(state.selected_tags.clone(), state.discover.loadedpages)?
                .items;
        state.discover.content.extend(discover);
    }
    Ok(())
}

pub fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init
    let stdout = Arc::new(FairMutex::new(stdout()));

    println!("Loading tags from bandcamp.com");
    let tags = tags::get_tags()?;
    println!("Loading gui...");

    {
        let stdout_clone = stdout.clone();
        let mut stdout = stdout_clone.lock();
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
        current_view: CurrentView::Tags,
        tags: ListBoxTag::default(),
        queue: ListBoxQueue::default(),
        selected_tags: Vec::new(),
        discover: ListBoxDiscover::default(),
        display_tags: true,
        diagnostics: ListBoxDiagnositcs::default(),
        selected_position: 0,
        queue_pos: 0,
    };

    state.tags.content = tags;
    redraw(&mut stdout.lock(), &mut state)?;

    let mut player = Player::new();

    state.print_diag("Player started!".to_string());

    loop {
        while !poll(Duration::from_millis(16))? {
            if let Some(time) = player.get_time() {
                if state.queue.content.len() > 0 {
                    let mins = state.queue.content[state.queue_pos].duration / 60.0;
                    let secs = state.queue.content[state.queue_pos].duration % 60.0;
                    state.bottom_text = format!(
                        "\r{}/{}:{} {} - {} pos: {} volume: {}%",
                        FormatTime(time),
                        mins as u32,
                        secs as u32,
                        state.queue.content[state.queue_pos].artist,
                        state.queue.content[state.queue_pos].title,
                        state.queue_pos,
                        (player.get_volume())
                    );

                    if (state.queue.content[state.queue_pos].duration - time.as_secs_f64()) < 1.0
                        && state.queue.content.len() - 1 > state.queue_pos
                    {
                        state.queue_pos += 1;
                        state.bottom_text = format!(
                            "Loading track: {} - {}",
                            state.queue.content[state.queue_pos].artist,
                            state.queue.content[state.queue_pos].title
                        );
                        player.switch_track(state.queue.content[state.queue_pos].audio_url.clone());
                    }
                }
            } else {
                state.bottom_text = format!("stopped volume: {}%", (player.get_volume()));
            }
            cli_drawing::redraw_bottom_bar(&mut stdout.lock(), &mut state)?;
        }

        let event = read()?;

        match event {
            Key(pressedkey) => {
                let (_cols, rows) =
                    size().expect("Unable to get terminal size continue work is not availble!");

                if pressedkey == KeyCode::Char('c').into() {
                    disable_raw_mode()?;
                    {
                        let mut stdout = stdout.lock();

                        stdout.queue(EnableBlinking)?;
                        stdout.queue(Show)?;
                        stdout.queue(Clear(ClearType::All))?;
                    }
                    break;
                }

                if pressedkey == KeyCode::Enter.into() {
                    if state.current_view == CurrentView::Tags {
                        if state.selected_tags.len() > 0 {
                            state.switch_view(&mut stdout.lock(), CurrentView::Albums)?;
                            while state.discover.content.len() < (rows - 2) as usize {
                                state.discover.loadedpages += 1;
                                let discover = album_parsing::get_tag_data(
                                    state.selected_tags.clone(),
                                    state.discover.loadedpages,
                                )?
                                .items;
                                state.discover.content.extend(discover);
                            }
                        }
                    }
                    if state.current_view == CurrentView::Albums {
                        let is_album = album_parsing::get_album(
                            state.discover.content[state.selected_position]
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
                                    state.queue.content.push(QueuedTrack {
                                        album: album
                                            .current
                                            .clone()
                                            .title
                                            .unwrap_or("Unknown album".to_string()),
                                        artist: state.discover.content[state.selected_position]
                                            .clone()
                                            .artist,
                                        title: album_track
                                            .title
                                            .unwrap_or("Unknown track title".to_string()),
                                        // TODO: switch to normal error-handling and not this garbage that panic...
                                        audio_url: album_track.file.unwrap().mp3128,
                                        album_url: album_url.clone(),
                                        duration: album_track.duration.unwrap_or(0.0),
                                    });
                                }
                            }
                            _ => state.status_bar(
                                format!(
                                    "Something went wrong while loading {}",
                                    state.discover.content[state.selected_position].title
                                ),
                                true,
                            ),
                        }
                    }
                    if state.current_view == CurrentView::Queue {
                        // TODO: Implement playback here
                        player.switch_track(
                            state.queue.content[state.selected_position]
                                .audio_url
                                .clone(),
                        );
                        state.queue_pos = state.selected_position;
                    }
                }

                if pressedkey == KeyCode::Char('d').into() {
                    &stdout.lock().execute(Clear(ClearType::All))?;
                    if state.current_view == CurrentView::Tags {
                        &state.selected_tags.clear();
                    }

                    if state.current_view == CurrentView::Albums {
                        state.cleanup_albums();
                    }

                    if state.current_view == CurrentView::Queue {
                        // stop playback
                        player.pause();
                        player.stop();
                        state.cleanup_queue();
                    }
                }

                if pressedkey == KeyCode::Char('x').into() {
                    state.switch_view(&mut stdout.lock(), CurrentView::Diagnositcs)?;
                }

                if pressedkey == KeyCode::Char('o').into() {
                    webbrowser::open(&state.queue.content[state.queue_pos].album_url);
                    // TODO: Add if error
                }

                if pressedkey == KeyCode::Char('h').into() {
                    state.display_tags = !state.display_tags;
                }

                if pressedkey == KeyCode::Char('q').into() {
                    &state.switch_view(&mut stdout.lock(), CurrentView::Queue);
                }

                if pressedkey == KeyCode::Tab.into() {
                    match state.current_view {
                        CurrentView::Albums => {
                            &state.switch_view(&mut stdout.lock(), CurrentView::Queue)
                        }
                        CurrentView::Tags => {
                            &state.switch_view(&mut stdout.lock(), CurrentView::Albums)
                        }
                        CurrentView::Queue => {
                            &state.switch_view(&mut stdout.lock(), CurrentView::Tags)
                        }
                        CurrentView::Diagnositcs => {
                            &state.switch_view(&mut stdout.lock(), CurrentView::Tags)
                        }
                    };
                }

                if pressedkey == KeyCode::Down.into() {
                    state.set_current_view_state(
                        &mut stdout.lock(),
                        state.selected_position + 1,
                        state.get_current_page(),
                    )?;
                    if state.selected_position > (rows - 3) as usize {
                        state.set_current_view_state(
                            &mut stdout.lock(),
                            0,
                            state.get_current_page(),
                        )?;
                        switch_page_up(&mut stdout.lock(), &mut state)?;
                    }
                }

                if pressedkey == KeyCode::Left.into() {
                    state.bottom_text = "Tracking back by 5 seconds... Please wait...".to_string();
                    player.seek_backward(Duration::from_secs(5));
                }

                if pressedkey == KeyCode::Char('w').into() {
                    if player.get_volume() < 99 {
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

                if pressedkey == KeyCode::Up.into() {
                    if state.selected_position > 0 {
                        state.set_current_view_state(
                            &mut stdout.lock(),
                            state.selected_position - 1,
                            state.get_current_page(),
                        )?;
                    } else {
                        if state.get_current_page() > 0 {
                            state.set_current_view_state(
                                &mut stdout.lock(),
                                state.selected_position,
                                state.get_current_page() - 1,
                            )?;
                        }
                        state.set_current_view_state(
                            &mut stdout.lock(),
                            (rows - 3) as usize,
                            state.get_current_page(),
                        )?;
                    }
                }

                if pressedkey == KeyCode::Char(' ').into() {
                    // TODO: if aready added - clear
                    if state.current_view == CurrentView::Tags {
                        state
                            .selected_tags
                            .push(state.tags.selected_tag_name.clone());
                    } else {
                        // TODO: Play pause goes here
                        player.set_paused(!player.is_paused());
                    }
                }

                redraw(&mut stdout.lock(), &mut state)?;
            }
            event::Event::Mouse(_) => {
                redraw(&mut stdout.lock(), &mut state)?;
            }
            event::Event::Resize(w, h) => {
                if w > 50 && h > 10 {
                    &stdout.lock().execute(Clear(ClearType::All))?;
                    redraw(&mut stdout.lock(), &mut state)?;
                    state.set_current_view_state(
                        &mut stdout.lock(),
                        0,
                        state.get_current_page(),
                    )?;
                } else {
                    &stdout.lock().execute(Clear(ClearType::All))?;
                    &stdout.lock().execute(cursor::MoveTo(0, 0))?;
                    &stdout
                        .lock()
                        .execute(style::PrintStyledContent("terminal is too small".red()))?;
                }
            }
        }
    }
    Ok(())
}

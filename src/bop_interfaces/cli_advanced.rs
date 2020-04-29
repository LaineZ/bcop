use crossterm::cursor::DisableBlinking;
use crossterm::event::read;
use crossterm::terminal::Clear;
use std::{sync::Arc, time::{Duration, Instant}, io::stdout};

use crate::bc_core;
use crate::bc_core::album_parsing;
use crate::bc_core::playback_advanced;

use crossterm::{cursor, event, QueueableCommand};
use crossterm::{
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, ClearType},
    ExecutableCommand,
};

use parking_lot::FairMutex;

use super::{
    cli_drawing::redraw,
    cli_structs::{
        CurrentView, ListBoxDiscover, ListBoxQueue, ListBoxTag, Playback, QueuedTrack, State,
    },
};

use anyhow::Result;

use bc_core::tags;
use cursor::{EnableBlinking, Hide, Show};
use event::{Event::Key, KeyCode};

async fn switch_page_up(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let idx = state.get_current_idx();
    let page = state.get_current_page();

    let (_cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    if page < (state.get_len() / (rows - 2) as usize) as usize {
        state.set_current_view_state(idx, page + 1);
    } else {
        state.status_bar("You aready scrolled to end!".to_string(), true);
    }

    // stream loading
    if state.current_view == CurrentView::Albums {
        state.status_bar("Loading next page...".to_string(), false);
        state.discover.loadedpages += 1;
        let discover = album_parsing::get_tag_data(
            state.selected_tags.clone()[0].clone(),
            state.discover.loadedpages,
        )
        .await?
        .items;
        state.discover.content.extend(discover);
    }
    Ok(())
}


fn bool_to_str(value: bool, on: &str, off: &str) -> String {
    if (value) { on.to_string() } else { off.to_string() }
}

pub async fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init
    let stdout = Arc::new(FairMutex::new(stdout()));

    println!("Loading tags from bandcamp.com");
    let tags = tags::get_tags().await?;
    println!("Loading gui...");

    {
        let stdout_clone = stdout.clone();
        let mut stdout = stdout_clone.lock();
        stdout.queue(DisableBlinking)?;
        stdout.queue(Hide)?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(event::EnableMouseCapture)?;
    }

    enable_raw_mode()?;
    let device = rodio::default_output_device().expect("Error opening output device!");
    let mut sink = rodio::Sink::new(&device);

    let mut state = State {
        statusbar_text: "[Space]: Select Tags [Enter]: Load tag albums".to_string(),
        error: false,
        current_view: CurrentView::Tags,
        tags: ListBoxTag::default(),
        queue: ListBoxQueue::default(),
        selected_tags: Vec::new(),
        discover: ListBoxDiscover::default(),
        display_tags: true,
    };

    let playback = Arc::new(FairMutex::new(Playback::default()));

    state.tags.content = tags;
    redraw(&mut stdout.lock(), &mut state)?;

    {
        let stdout = stdout.clone();
        let playback = playback.clone();

        std::thread::spawn(move || -> Result<()> {
            loop {
                std::thread::sleep(Duration::from_secs(1));

                let (_cols, rows) =
                    size().expect("Unable to get terminal size continue work is not availble!");
                let playback = playback.lock();
                if !playback.is_paused {
                    let mut time = playback.started_at.elapsed() - playback.pause_duration;
                    if let Some(paused_at) = playback.paused_at {
                        time -= paused_at.elapsed();
                    }

                    let min = time.as_secs() / 60;
                    let sec = time.as_secs() % 60;
                    let mut stdout = stdout.lock();

                    &stdout
                        .execute(cursor::MoveTo(0, rows))?
                        .execute(Print(format!(
                            "{}:{:02}: {} - {}",
                            min, sec, playback.currently_playing.artist, playback.currently_playing.title
                        )));
                } else {
                    let mut stdout = stdout.lock();
                    &stdout
                        .execute(cursor::MoveTo(0, rows))?
                        .execute(Print("Playback is paused"));
                }
            }
        });
    }

    loop {
        if !sink.empty() {
            let mut playback = playback.lock();
            playback.is_paused = sink.is_paused();
            drop(playback);
        }

        match read()? {
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
                        stdout.queue(event::DisableMouseCapture)?;
                    }
                    break;
                }

                if pressedkey == KeyCode::Enter.into() {
                    if state.current_view == CurrentView::Tags {
                        if state.selected_tags.len() > 0 {
                            state.statusbar_text = format!("Discovering");
                            state.switch_view(CurrentView::Albums);
                            while state.discover.content.len() < (rows - 2) as usize {
                                state.discover.loadedpages += 1;
                                let discover = album_parsing::get_tag_data(
                                    state.selected_tags.clone()[0].clone(),
                                    state.discover.loadedpages,
                                )
                                .await?
                                .items;
                                state.discover.content.extend(discover);
                            }
                            state.statusbar_text = format!("Done!");
                        }
                    }
                    if state.current_view == CurrentView::Albums {
                        let is_album = album_parsing::get_album(
                            state.discover.content[state.discover.selected_idx]
                                .tralbum_url
                                .as_str(),
                        )
                        .await;

                        match is_album {
                            Some(album) => {
                                for album_track in album.trackinfo.unwrap() {
                                    state.queue.content.push(QueuedTrack {
                                        album: album
                                            .current
                                            .clone()
                                            .title
                                            .unwrap_or("Unknown album".to_string()),
                                        artist: album
                                            .current
                                            .clone()
                                            .artist
                                            .unwrap_or("Unknown artist".to_string()),
                                        title: album_track
                                            .title
                                            .unwrap_or("Unknown track title".to_string()),
                                        // TODO: switch to normal error-handling and not this garbage that panic...
                                        audio_url: album_track.file.unwrap().mp3128,
                                    });
                                }
                            }
                            _ => state.status_bar(
                                format!(
                                    "Something went wrong while loading {}",
                                    state.discover.content[state.discover.selected_idx].title
                                ),
                                true,
                            ),
                        }
                    }
                    if state.current_view == CurrentView::Queue {
                        playback.lock().currently_playing =
                            state.queue.content[state.get_current_idx()].clone();
                        let bytes = bc_core::playback::get_track_from_url(
                            playback.lock().currently_playing.audio_url.as_str(),
                        )
                        .await?;
                        let device =
                            rodio::default_output_device().expect("Error opening output device!");
                        sink = playback_advanced::create_sink(bytes, device, 0)?;
                        playback.lock().started_at = Instant::now();
                        sink.play();
                    }
                }

                if pressedkey == KeyCode::Char('d').into() {
                    &state.selected_tags.clear();
                }

                if pressedkey == KeyCode::Char('h').into() {
                    state.display_tags = !state.display_tags;
                }

                if pressedkey == KeyCode::Char('q').into() {
                    &state.switch_view(CurrentView::Queue);
                }

                if pressedkey == KeyCode::Tab.into() {
                    if state.current_view == CurrentView::Albums {
                        &state.switch_view(CurrentView::Tags);
                    } else {
                        &state.switch_view(CurrentView::Albums);
                    };
                }

                if pressedkey == KeyCode::Down.into() {
                    state.set_current_view_state(
                        state.get_current_idx() + 1,
                        state.get_current_page(),
                    );
                    if state.get_current_idx() > (rows - 3) as usize {
                        state.set_current_view_state(0, state.get_current_page());
                        switch_page_up(&mut state).await?;
                    }
                }

                if pressedkey == KeyCode::Up.into() {
                    if state.get_current_idx() > 0 {
                        state.set_current_view_state(
                            state.get_current_idx() - 1,
                            state.get_current_page(),
                        );
                    } else {
                        if state.get_current_page() > 0 {
                            state.set_current_view_state(
                                state.get_current_idx(),
                                state.get_current_page() - 1,
                            );
                        }
                        state.set_current_view_state((rows - 3) as usize, state.get_current_page());
                    }
                }

                if pressedkey == KeyCode::Char(' ').into() {
                    // TODO: if aready added - clear
                    if state.current_view == CurrentView::Tags {
                        state
                            .selected_tags
                            .push(state.tags.selected_tag_name.clone());
                    } else {
                        if sink.is_paused() {
                            let mut playback = playback.lock();

                            if let Some(instant) = playback.paused_at {
                                playback.pause_duration += instant.elapsed();
                                playback.paused_at = None;
                            }
                            sink.play();
                        } else {
                            sink.pause();
                            playback.lock().paused_at = Some(Instant::now());
                        }
                    }
                }

                redraw(&mut stdout.lock(), &mut state)?;
            }
            event::Event::Mouse(_) => {
                redraw(&mut stdout.lock(), &mut state)?;
            }
            event::Event::Resize(_, _) => {
                redraw(&mut stdout.lock(), &mut state)?;
                state.set_current_view_state(0, state.get_current_page());
            }
        }
    }

    Ok(())
}

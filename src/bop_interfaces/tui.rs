use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::bc_core::{
    playback::{FormatTime, Player},
    queue::Queue,
};

use super::{listbox::ListBox, statebar::StateBar, tui_structs::State};
use console_engine::{Color, KeyCode, crossterm::{
        event::{self, read},
        terminal::size,
    }};

const LIST_TAGS: usize = 0;
const LIST_DISCOVER: usize = 1;
const LIST_QUEUE: usize = 2;

fn setup_focus_at(id: usize, lbx: &mut Vec<ListBox>, bar: &mut StateBar) {
    for list in lbx.iter_mut() {
        list.focused = false;
    }
    lbx[id].focused = true;
    bar.information(&lbx[id].description);
}

fn get_focus_at(lbx: &mut Vec<ListBox>) -> usize {
    for (id, list) in lbx.iter_mut().enumerate() {
        if list.focused {
            return id;
        }
    }
    0
}

pub fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Tag loading
    let tags: Vec<String> = include_str!("tags.list")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();

    let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");
    let mut debug_overlay = false;

    let mut listboxes = Vec::new();
    listboxes.push(ListBox::new(cols, rows, true, "Tags"));
    listboxes.push(ListBox::new(cols, rows, false, "Discover"));
    listboxes.push(ListBox::new(cols, rows, false, "Play queue"));
    listboxes[LIST_TAGS].display.extend(tags);

    let mut player = Player::new();

    let mut state = State::new();
    let mut bar = StateBar::new();
    let mut queue = Queue::new();


    let mut engine = console_engine::ConsoleEngine::init(cols as u32, rows as u32, 30);

    let mut stopwatch = std::time::Instant::now();
    let mut last_fps = 0;

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize();
        engine.clear_screen(); // reset the screen
        for list in listboxes.iter_mut() {
            if list.focused {
                engine.print_screen(0, 0, list.draw());
            }
        }
        engine.print_screen(0, bar.y as i32, bar.draw());

        if debug_overlay {
            engine.print_fbg(
                1,
                1,
                format!(
                    "Terminal size: {}x{} FPS: {}",
                    engine.get_width(),
                    engine.get_height(),
                    last_fps,
                )
                .as_str(),
                Color::White,
                Color::DarkBlue
            );
        }

        engine.draw();

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        if engine.is_key_held(KeyCode::Down) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.scroll_down();
                }
            }
        }

        if engine.is_key_held(KeyCode::Left) {
            player.seek_backward(Duration::from_secs(5));
        }

        if engine.is_key_held(KeyCode::Right) {
            player.seek_forward(Duration::from_secs(5));
        }

        if engine.is_key_held(KeyCode::Up) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.scroll_up();
                }
            }
        }

        if engine.is_key_pressed(KeyCode::Char('l')) {
            state.extend_discover().unwrap();
            for data in state.discover.iter_mut() {
                listboxes[LIST_DISCOVER]
                    .display
                    .push(format!("{} - {}", data.artist, data.title))
            }
        }

        if engine.is_key_pressed(KeyCode::Enter) {
            if listboxes[LIST_TAGS].focused {
                state
                    .selected_tags
                    .push(listboxes[LIST_TAGS].get_selected_str());

                state.extend_discover()?;
                for data in state.discover.iter_mut() {
                    listboxes[LIST_DISCOVER]
                        .display
                        .push(format!("{} - {}", data.artist, data.title))
                }
                setup_focus_at(LIST_DISCOVER, &mut listboxes, &mut bar);
            }

            if listboxes[LIST_DISCOVER].focused {
                let artist = state.discover[listboxes[LIST_DISCOVER].get_selected_idx()]
                    .artist
                    .clone();
                let url = state.discover[listboxes[LIST_DISCOVER].get_selected_idx()]
                    .tralbum_url
                    .clone();
                queue.add_album_in_queue(artist, url.as_str()).unwrap();

                for data in queue.queue.iter_mut() {
                    listboxes[LIST_QUEUE].display.push(data.to_string())
                }
            }

            if listboxes[LIST_QUEUE].focused {
                queue.set(listboxes[LIST_QUEUE].get_selected_idx());
                player.switch_track(queue.get_current_track().unwrap().audio_url);
            }
        }

        if engine.is_key_pressed(KeyCode::Tab) {
            let switch = get_focus_at(&mut listboxes) + 1;
            if switch > listboxes.len() - 1 {
                setup_focus_at(LIST_TAGS, &mut listboxes, &mut bar);
            } else {
                setup_focus_at(switch, &mut listboxes, &mut bar);
            }
        }

        if engine.is_key_pressed(KeyCode::F(1)) {
            debug_overlay = !debug_overlay;
        }

        if engine.is_key_pressed(KeyCode::Char(' ')) {
            player.set_paused(!player.is_paused());
        }

        if engine.is_key_pressed(KeyCode::Char('o')) {
            if queue.queue.len() > 0 {
                webbrowser::open(&queue.get_current_track().unwrap().album_url)?;
            } else {
                bar.error("Queue list is empty!");
            }
        }


        match engine.get_resize() {
            Some((width, height)) => {
                for list in listboxes.iter_mut() {
                    list.resize(width, height);
                }
                bar.resize(width, height);
            }

            None => {}
        }

        // TODO: change this
        {
            match player.get_time() {
                Some(time) => match queue.get_current_track() {
                    Some(track) => {
                        if time >= track.duration {
                            bar.bottom_info("Loading next track...");
                            queue.next();
                            player.switch_track(queue.get_current_track().unwrap().audio_url);
                        }

                        let mut state_pl = "◼";
                        if player.is_paused() {
                            state_pl = "⏸"
                        } else {
                            state_pl = "▶"
                        }

                        bar.bottom_info(format!(
                            "{} {} - {} from {} {}/{}",
                            state_pl,
                            track.artist,
                            track.title,
                            track.album,
                            FormatTime(player.get_time().unwrap_or(Duration::from_secs(0))),
                            FormatTime(track.duration)
                        ));
                    }
                    None => {
                        bar.bottom_info("Queue does not contain any track");
                    }
                },

                None => {
                    bar.bottom_info("Playback stopped");
                }
            }
        }

        if stopwatch.elapsed().as_millis() >= 1000 {
            last_fps = engine.frame_count;
            engine.frame_count = 0;
            stopwatch = std::time::Instant::now();
        }
    }
    Ok(())
}

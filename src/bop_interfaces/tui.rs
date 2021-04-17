use std::time::Duration;

use crate::bc_core::{
    playback::{FormatTime, Player},
    queue::Queue,
};

use super::{listbox::ListBox, statebar::StateBar, statebar::StateBarWidget, tui_structs::State};
use console_engine::{Color, ConsoleEngine, KeyCode, MouseButton};

const LIST_TAGS: usize = 0;
const LIST_DISCOVER: usize = 1;
const LIST_QUEUE: usize = 2;

const STATEBAR_PROGRAMHEADER: usize = 0;
const STATEBAR_VOLUME: usize = 1;
const STATEBAR_TRACK_INFO: usize = 2;
const STATEBAR_TRACK_SEEKBAR: usize = 3;
const STATEBAR_SHUFFLE: usize = 4;

/// Change this varible to speed up rendering
pub const MAX_FPS: u32 = 30;

fn setup_focus_at(id: usize, lbx: &mut Vec<ListBox>, bar: &mut StateBar) {
    for list in lbx.iter_mut() {
        list.focused = false;
    }
    lbx[id].focused = true;
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

    let mut engine = console_engine::ConsoleEngine::init_fill(MAX_FPS);

    let (cols, rows) = (engine.get_width() as u16, engine.get_height() as u16);
    let mut debug_overlay = false;

    // LISTBOXES
    let mut listboxes = Vec::new();
    listboxes.push(ListBox::new(cols, rows, true, "Tags"));
    listboxes.push(ListBox::new(cols, rows, false, "Discover"));
    listboxes.push(ListBox::new(cols, rows, false, "Play queue"));
    listboxes[LIST_TAGS].display.extend(tags);

    // STATEBAR
    let mut bar = StateBar::new();
    // adding 4 statebars widgets
    for _ in 0..4 {
        bar.widgets.push(StateBarWidget::new());
    }

    bar.widgets[STATEBAR_PROGRAMHEADER].set_text("▶ BandcampOnlinePlayer RS");

    let mut player = Player::new();

    let mut state = State::new();
    let mut queue = Queue::new();

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
                Color::DarkBlue,
            );
        }

        engine.draw();

        if engine.is_key_pressed(KeyCode::Esc)
            || engine.is_key_pressed_with_modifier(
                KeyCode::Char('c'),
                console_engine::KeyModifiers::CONTROL,
            )
        {
            break;
        }

        if engine.is_key_held(KeyCode::Down) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.scroll_down();
                }
            }
        }

        if engine.is_key_held(KeyCode::Up) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.scroll_up();
                }
            }
        }

        if engine.is_key_pressed(KeyCode::Char('l')) {
            state.extend_discover(listboxes[LIST_TAGS].highlight.clone())?;
            for data in state.discover.iter_mut() {
                listboxes[LIST_DISCOVER]
                    .display
                    .push(format!("{} - {}", data.artist, data.title))
            }
        }

        if engine.is_key_pressed(KeyCode::Char('d')) {}

        if engine.is_key_pressed(KeyCode::Enter) {
            if listboxes[LIST_TAGS].focused {
                if !listboxes[LIST_TAGS].highlight.is_empty() {
                    bar.widgets[STATEBAR_TRACK_INFO].set_text("Loading discover...");
                    setup_focus_at(LIST_DISCOVER, &mut listboxes, &mut bar);
                    state.extend_discover(listboxes[LIST_TAGS].highlight.clone())?;
                    for data in state.discover.iter_mut() {
                        listboxes[LIST_DISCOVER]
                            .display
                            .push(format!("{} - {}", data.artist, data.title))
                    }
                } else {
                    bar.widgets[STATEBAR_TRACK_INFO].set_text("Please select at least 1 tag!");
                }
            }

            if listboxes[LIST_DISCOVER].focused {
                if !state.discover.is_empty() {
                    let url = state.discover[listboxes[LIST_DISCOVER].get_selected_idx()]
                        .tralbum_url
                        .clone();
                    queue.add_album_in_queue(url.as_str())?;

                    for data in queue.queue.iter_mut() {
                        listboxes[LIST_QUEUE].display.push(data.to_string())
                    }
                } else {
                    bar.widgets[STATEBAR_TRACK_INFO].set_text("Can't load discover... Please select another tags");
                }
            }

            if listboxes[LIST_QUEUE].focused {
                player.switch_track(
                    queue
                        .set(listboxes[LIST_QUEUE].get_selected_idx())
                        .unwrap()
                        .audio_url,
                );
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

        if engine.is_key_pressed(KeyCode::F(2)) {
            debug_overlay = !debug_overlay;
        }

        // player controls
        if engine.is_key_pressed(KeyCode::Char(' ')) {
            if listboxes[LIST_TAGS].focused {
                let tag = listboxes[LIST_TAGS].clone().get_selected_str();
                listboxes[LIST_TAGS].highlight(tag.as_str());
            } else {
                player.set_paused(!player.is_paused());
            }
        }

        if engine.is_key_held(KeyCode::Left) {
            player.seek_backward(Duration::from_secs(5));
        }

        if engine.is_key_held(KeyCode::Right) {
            player.seek_forward(Duration::from_secs(5));
        }

        if engine.is_key_pressed(KeyCode::Char('s')) {
            player.stop();
        }

        if engine.is_key_held(KeyCode::Char('a')) {
            if player.get_volume() > 0 {
                player.decrease_volume(5);
            }
        }

        if engine.is_key_held(KeyCode::Char('d')) {
            if player.get_volume() < 100 {
                player.increase_volume(5);
            }
        }

        if let Some((x, y)) = engine.get_mouse_press(MouseButton::Left) {
            let (_cols, rows) = (engine.get_width(), engine.get_height());
            // pause
            if x == 0 && y == rows as u32 - 1 {
                player.set_paused(!player.is_paused());
            }
            for lists in listboxes.iter_mut() {
                if x < lists.screen.get_width() && y < rows - 2 {
                    lists.position = y as usize;
                }
            }
        }

        if engine.is_mouse_scrolled_down() || engine.is_key_held(KeyCode::PageDown) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.switch_page_up();
                }
            }
        }

        if engine.is_mouse_scrolled_up() || engine.is_key_held(KeyCode::PageUp) {
            for list in listboxes.iter_mut() {
                if list.focused {
                    list.switch_page_down();
                }
            }
        }

        if engine.is_key_pressed(KeyCode::Char('o')) {
            if !queue.queue.is_empty() {
                webbrowser::open(&queue.get_current_track().unwrap().album_url)?;
            } else {
                bar.widgets[STATEBAR_TRACK_INFO].set_text("Queue list is empty!");
            }
        }

        if let Some((width, height)) = engine.get_resize() {
            for list in listboxes.iter_mut() {
                list.resize(width, height);
            }
            bar.resize(width, height);
        }

        // TODO: change this
        match player.get_time() {
            Some(time) => {
                if let Some(track) = queue.get_current_track() {
                    if time >= track.duration {
                        bar.widgets[STATEBAR_TRACK_INFO].set_text("Loading next track...");
                        if let Some(track) = queue.next() {
                            player.switch_track(track.audio_url);
                        } else {
                            player.stop();
                            bar.widgets[STATEBAR_TRACK_INFO].set_text("Finished playback");
                        }
                    }
                }
            }

            None => {}
        }

        if stopwatch.elapsed().as_millis() >= 1000 {
            last_fps = engine.frame_count;
            engine.frame_count = 0;
            stopwatch = std::time::Instant::now();
        }
    }
    Ok(())
}

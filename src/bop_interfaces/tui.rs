use std::time::Duration;

use super::{listbox::ListBox, tui_structs::State, statebar::StateBar};
use console_engine::{
    crossterm::{
        event::{self, read},
        terminal::size,
    },
    KeyCode,
};

const LIST_TAGS: usize = 0;
const LIST_DISCOVER: usize = 1;
const LIST_QUEUE: usize = 3;


fn setup_focus_at(id: usize, lbx: &mut Vec<ListBox>) {
    for list in lbx.iter_mut() {
        list.focused = false;
    }
    lbx[id].focused = true;
}

fn get_focus_at(lbx: &mut Vec<ListBox>) -> usize {
    for (id, list) in lbx.iter_mut().enumerate() {
        if list.focused {
            return id
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
    listboxes.push(ListBox::new(cols, rows, true));
    listboxes.push(ListBox::new(cols, rows, false));
    listboxes.push(ListBox::new(cols, rows, false));
    listboxes[LIST_TAGS].display.extend(tags);
    let mut state = State::new();
    let mut bar = StateBar::new();

    let mut engine = console_engine::ConsoleEngine::init(cols as u32, rows as u32, 30);

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
            engine.print(1, 1, format!("Terminal size: {}x{}", engine.get_width(), engine.get_height()).as_str());
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
            log::info!("Enter pressed");
            if listboxes[LIST_TAGS].focused {

                state.selected_tags.push(listboxes[LIST_TAGS].get_selected_str());

                state.extend_discover()?;
                for data in state.discover.iter_mut() {
                    listboxes[LIST_DISCOVER]
                        .display
                        .push(format!("{} - {}", data.artist, data.title))
                }

                setup_focus_at(LIST_DISCOVER, &mut listboxes);
            }
        }

        if engine.is_key_pressed(KeyCode::Tab) {
            let switch = get_focus_at(&mut listboxes) + 1;
            if switch > listboxes.len() - 1 {
                setup_focus_at(LIST_TAGS, &mut listboxes);
            } else {
                setup_focus_at(switch, &mut listboxes);
            }
        }

        if engine.is_key_pressed(KeyCode::F(1)) {
            debug_overlay =! debug_overlay;
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
    }
    Ok(())
}

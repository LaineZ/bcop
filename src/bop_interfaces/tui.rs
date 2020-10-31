use super::{
    listbox::ListBox,
    tui_structs::State,
};
use console_engine::{crossterm::terminal::size, KeyCode};

const LIST_TAGS: usize = 0;
const LIST_DISCOVER: usize = 1;

pub fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Tag loading
    let tags: Vec<String> = include_str!("tags.list")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();

    // Terminal configuration
    println!("Loading TUI...");

    let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");

    let mut listbox = ListBox::new(cols, rows);
    listbox.display.extend(tags);

    let mut current_select = LIST_TAGS;
    let mut state = State::new();

    let mut engine = console_engine::ConsoleEngine::init(cols as u32, rows as u32, 30);

    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize();
        engine.clear_screen(); // reset the screen
        engine.set_screen(listbox.draw());
        engine.draw();

        if engine.is_key_pressed(KeyCode::Esc) {
            break;
        }

        if engine.is_key_held(KeyCode::Down) {
            listbox.scroll_down();
        }


        if engine.is_key_held(KeyCode::Up) {
            listbox.scroll_up();
        }

        if engine.is_key_pressed(KeyCode::Char('l')) {
            state.extend_discover()?;
            for data in state.discover.iter_mut() {
                listbox.display.push(format!("{} - {}", data.artist, data.title))
            }
        }

        if engine.is_key_pressed(KeyCode::Enter) {
            if current_select == LIST_TAGS {
                current_select = LIST_DISCOVER;
                state.selected_tags.push(listbox.display[listbox.position].clone());
                listbox.display.clear();
                state.extend_discover()?;
                for data in state.discover.iter_mut() {
                    listbox.display.push(format!("{} - {}", data.artist, data.title))
                }
            }
        }

    }
    Ok(())
}

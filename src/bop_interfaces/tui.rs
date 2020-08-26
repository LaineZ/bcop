use super::{tui_drawing::{redraw, ListBox}, tui_structs::State};
use crossterm::{
    cursor::{DisableBlinking, Hide},
    terminal::{enable_raw_mode, size, Clear, ClearType},
    QueueableCommand,
};
use std::io::stdout;
use console_fb::FrameBuffer;

// Listbox constants =)
const COLS_COUNT: u16 = 2;
const LIST_BOX_TAGS: usize = 0;
const LIST_BOX_DISCOVER: usize = 1;
const LIST_BOX_QUEUE: usize = 2;

pub fn loadinterface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // Terminal initialization
    let mut stdout = stdout();
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not available!");
    log::info!("Detected terminal size {}x{}", cols, rows);

    // Tag loading
    let tags: Vec<String> = include_str!("tags.list")
        .split("\n")
        .map(|s| s.trim().to_string())
        .collect();

    // Terminal configuration
    println!("Loading TUI...");
    stdout.queue(DisableBlinking)?;
    stdout.queue(Hide)?;
    stdout.queue(Clear(ClearType::All))?;
    // Linux only feature
    enable_raw_mode()?;

    let mut listboxes = Vec::new();
    // init listboxes (PLEASE KEEP ORDER WITH CONSTANTS)
    // tags
    listboxes.push(ListBox::new(15, rows - 1, 0, true));
    listboxes[LIST_BOX_TAGS].add_range(tags);

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

    let state = State::new();

    let mut framebuffer = FrameBuffer::create(cols, rows);

    redraw(&state, &mut framebuffer, &mut listboxes)?;
    framebuffer.push_fb(&mut stdout, true);
    Ok(())
}

use crossterm::style::SetBackgroundColor;
use crossterm::terminal::Clear;

use crossterm::{cursor, QueueableCommand};
use crossterm::{
    style::{self, Print},
    terminal::{size, ClearType},
    ExecutableCommand,
};

use super::cli_structs::{CurrentView, State};

use anyhow::Result;
use style::{Color, SetForegroundColor};

pub fn redraw(stdout: &mut std::io::Stdout, state: &mut State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    let mut lineheight = state
        .tags
        .content
        .iter()
        .max_by_key(|p| p.len())
        .unwrap()
        .len() as u16;

    // TODO: Refactor

    let lineheight_album = state
        .discover
        .content
        .iter()
        .max_by_key(|p| format!("{} by {}", p.title, p.artist).len());
    let mut lineheight_album_int: u16 = lineheight;
    match lineheight_album {
        Some(value) => {
            lineheight_album_int += format!("{} by {}", value.title, value.artist).len() as u16
        }
        None => lineheight_album_int += 20,
    }

    let lineheight_queue = state
        .queue
        .content
        .iter()
        .max_by_key(|p| format!("{} - {}", p.title, p.artist).len());
    let mut lineheight_queue_int: u16 = lineheight_album_int;
    match lineheight_queue {
        Some(value) => {
            lineheight_queue_int += format!("{} by {}", value.title, value.artist).len() as u16
        }
        None => lineheight_queue_int += 20,
    }

    let pages = state.tags.content.chunks((rows - 2) as usize);
    let album_pages = state.discover.content.chunks((rows - 2) as usize);
    let queue_pages = state.queue.content.chunks((rows - 2) as usize);

    stdout.queue(Clear(ClearType::All))?;

    if state.display_tags {
        for (i, v) in &mut pages.into_iter().enumerate() {
            if i == state.tags.selected_page {
                for (index, page) in v.into_iter().enumerate() {
                    if index == state.tags.selected_idx && state.current_view == CurrentView::Tags {
                        &stdout.execute(SetBackgroundColor(Color::White))?;
                        &stdout.execute(SetForegroundColor(Color::Black))?;
                        let page_str = page.to_string();
                        state.tags.selected_tag_name = page_str;
                    }

                    if state.selected_tags.iter().any(|i| i == page) {
                        &stdout.execute(SetForegroundColor(Color::Red))?;
                    }

                    if state.current_view != CurrentView::Tags {
                        &stdout.execute(SetForegroundColor(Color::Grey))?;
                    }

                    &stdout
                        .queue(cursor::MoveTo(0, (index + 1) as u16))?
                        .queue(Print(page))?;
                    &stdout.execute(style::ResetColor)?;
                }
            }
        }
    } else {
        lineheight = 0;
    }

    for (i, v) in &mut album_pages.into_iter().enumerate() {
        if i == state.discover.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.discover.selected_idx {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                    //state.selected_tag_name = page_str;
                }

                if state.current_view != CurrentView::Albums {
                    &stdout.execute(SetForegroundColor(Color::Grey))?;
                }

                let formatting = format!("{} by {}", page.clone().title, page.clone().artist);
                &stdout
                    .queue(cursor::MoveTo(lineheight + 1, (index + 1) as u16))?
                    .queue(Print(formatting))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }

    for (i, v) in &mut queue_pages.into_iter().enumerate() {
        if i == state.queue.selected_page {
            for (index, page) in v.into_iter().enumerate() {
                if index == state.queue.selected_idx {
                    &stdout.execute(SetBackgroundColor(Color::White))?;
                    &stdout.execute(SetForegroundColor(Color::Black))?;
                }

                if state.current_view != CurrentView::Queue {
                    &stdout.execute(SetForegroundColor(Color::Grey))?;
                }

                let formatting = format!("{} - {}", page.clone().title, page.clone().artist);
                &stdout
                    .queue(cursor::MoveTo(lineheight_album_int + 1, (index + 2) as u16))?
                    .queue(Print(formatting))?;
                &stdout.execute(style::ResetColor)?;
            }
        }
    }

    // drawing lines
    state.draw_line(stdout, lineheight)?;
    state.draw_line(stdout, lineheight_album_int)?;
    state.draw_line(stdout, lineheight_queue_int)?;

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

    &stdout.execute(cursor::MoveTo(0, 0))?.execute(Print(format!(
        "▶ BandcampOnlinePlayer RS | {}{}",
        &state.statusbar_text,
        " ".repeat(fixed_space as usize)
    )));
    &stdout.execute(style::ResetColor)?;
    Ok(())
}

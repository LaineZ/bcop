use crossterm::event::read;
use crossterm::style::SetBackgroundColor;
use crossterm::terminal::Clear;
use crossterm::cursor::DisableBlinking;
use std::time::{Duration, Instant};

use crate::bop_core;
use crate::bop_core::album_parsing;
use crate::bop_core::playback;
use crate::bop_core::playback_advanced;
use crate::model::album;
use bytes::Bytes;
use std::io::{stdout, Write, Stdout, self};
use crossterm::{execute, ExecutableCommand, terminal::{size, ClearType}, style::{self, Colorize, Print}};
use crossterm::{event, cursor, QueueableCommand};

use anyhow::Result;
use cursor::Hide;
use style::Color;
use bop_core::tags;
use event::{Event::{self, Key}, KeyEvent, KeyCode};


struct State {
    statusbar_text: &'static str
}

fn redraw(stdout: &mut std::io::Stdout, tags: &Vec<String>, state: &State) -> Result<()> {
    let (cols, rows) = size().expect("Unable to get terminal size continue work is not availble!");

    let mut idx: usize = 0;

    let pages = tags.chunks((rows - 2) as usize);

    stdout.queue(Clear(ClearType::All))?;


    for (i, v) in &mut pages.into_iter().enumerate() {
        if i == idx {
            for (index, page) in v.into_iter().enumerate() {
                &stdout.queue(cursor::MoveTo(0,(index + 1) as u16))?.queue(style::Print(page))?;
            }
        }
    }

    &stdout.execute(SetBackgroundColor(Color::Blue))?;
    &stdout.execute(cursor::MoveTo(0,0))?.execute(Print(format!("▶ BandcampOnlinePlayer RS | {}{}", &state.statusbar_text, " ".repeat(state.statusbar_text.len() as u16 - cols))));
    &stdout.execute(style::ResetColor)?;
    Ok(())
}


pub async fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // init

    let mut stdout = stdout();

    stdout.queue(DisableBlinking)?;
    stdout.queue(Hide)?;
    stdout.queue(Clear(ClearType::All))?;
    stdout.queue(event::EnableMouseCapture)?;

    let mut state = State { statusbar_text: "ДИДЖЕЙ МАДЕСТ!"};

    let tags = tags::get_tags().await?;

    redraw(&mut stdout, &tags, &state)?;

    loop {
        match read()? {
            Key(pressedkey) => { 
                if pressedkey == KeyCode::Char('c').into() {
                   break;
                }
            }
            event::Event::Mouse(_) => { redraw(&mut stdout, &tags.clone(), &state)?; }
            event::Event::Resize(_, _) => {  redraw(&mut stdout, &tags.clone(), &state)?; }
        }
    }

    Ok(())
}
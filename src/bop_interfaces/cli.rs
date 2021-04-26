use std::{io::Write, time::Duration};

use crate::{
    bc_core::{self, queue::Queue},
    model::search,
};
use anyhow::Result;
use bc_core::{album_parsing::search, playback::FormatTime};

fn loop_control(
    queue: &mut Queue,
    search_results: &mut Vec<search::Result>,
) -> Result<()> {
    let mut command = String::new();

    print!("}} ");
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut command)?;

    let command_args: Vec<&str> = command.as_str().trim().split(' ').collect();
    match command_args[0] {
        "c" => std::process::exit(0),
        "exit" => std::process::exit(0),
        "voli" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u16>() {
                    Ok(volume) => {
                        queue.player.increase_volume(volume);
                        println!("volume set to: {}%", queue.player.get_volume())
                    }
                    Err(_) => println!("error: invalid volume format"),
                }
            }
        }

        "vold" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u16>() {
                    Ok(volume) => {
                        queue.player.decrease_volume(volume);
                        println!("volume set to: {}%", queue.player.get_volume())
                    }
                    Err(_) => println!("error: invalid volume format"),
                }
            }
        }

        "seekf" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u64>() {
                    Ok(seek) => {
                        println!("seeking forward to {} seconds", seek);
                        queue.player.seek_forward(Duration::from_secs(seek));
                    }
                    Err(_) => println!("error: invalid seek format"),
                }
            }
        }

        "seek" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u64>() {
                    Ok(seek) => {
                        println!("seeking to {} seconds", seek);
                        queue.player.seek(Duration::from_secs(seek));
                    }
                    Err(_) => println!("error: invalid seek format"),
                }
            }
        }

        "stop" => {
            queue.player.stop();
            println!("queue.player stopped!");
        }

        "seekb" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u64>() {
                    Ok(seek) => {
                        println!("seeking backwards to {} seconds", seek);
                        queue.player.seek_backward(Duration::from_secs(seek));
                    }
                    Err(_) => println!("error: invalid seek format"),
                }
            }
        }

        "del" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<usize>() {
                    Ok(idx) => {
                        if idx < queue.queue.len() {
                            queue.queue.remove(idx);
                            println!("removed queued track with position: {}", idx);
                        } else {
                            println!("error: incorrect queue index!");
                        }
                    }
                    Err(_) => println!("error: delete position must be integer index"),
                }
            }
        }

        "search" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                if let Some(mut search_res) = search(command_args[1]) {
                    search_results.clear();
                    search_results.append(&mut search_res.results);
                    for (i, res) in search_results.iter().enumerate() {
                        if res.field_type == "t" || res.field_type == "a" {
                            println!(
                                "[{}] {} - {} @ {}",
                                i.saturating_sub(1),
                                res.band_name.as_ref().unwrap_or(&String::new()),
                                res.name.as_ref().unwrap_or(&String::new()),
                                res.url
                            );
                        }
                    }
                    println!("type `ins [index]` to add search result content to queue");
                } else {
                    println!("not found");
                }
            }
        }

        "ins" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<usize>() {
                    Ok(id) => {
                        // load from search
                        let ent_idx = id.clamp(0, search_results.len().saturating_sub(1));
                        if !search_results.is_empty() {
                            let url = &search_results[ent_idx].url;
                            &queue.add_album_in_queue(url).unwrap();
                        } else {
                            println!("empty!");
                            log::info!("VALUES: {:#?}", search_results);
                        }
                    }
                    Err(_) => {
                        // load url
                        if command_args[1].starts_with("http") {
                            if queue.add_album_in_queue(command_args[1]).is_err() {
                                println!("error while parsing album data!");
                            } else {
                                println!("track(s) inserted sucessfully type 'ls' to view it!");
                            }
                        }
                    }
                }
            }
        }

        "ls" => {
            if queue.queue.is_empty() {
                println!("queue is empty!");
            }
            for (i, track) in queue.queue.iter().enumerate() {
                if queue.queue_pos == i {
                    println!(
                        "> {} {} - {} from {}",
                        i, track.artist, track.title, track.album
                    );
                } else {
                    println!(
                        "  {} {} - {} from {}",
                        i, track.artist, track.title, track.album
                    );
                }
            }
        }

        "p" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                {
                    match command_args[1].parse::<usize>() {
                        Ok(idx) => {
                            queue.queue_pos = idx;
                            queue.start();
                        }
                        Err(_) => {
                            println!("incorrect format!");
                        }
                    }
                }
            } else {
                if !queue.player.is_paused() {
                    println!("info: pause");
                    queue.player.set_paused(true);
                } else {
                    println!("info: playing");
                    queue.player.set_paused(false);
                }
            }
        }

        "d" => {
            if let Some(duration) = queue.player.get_time() {
                println!("{}", FormatTime(duration));
            }
        }

        "next" => {
            queue.next();
            println!("next track!");
        }

        "prev" => {
            queue.prev();
            println!("prev track!");
        }

        "help" => {
            println!("command help:");
            println!("`c`, `exit` - closes program");
            println!("`p` - play/pause");
            println!("`d` - current track duration");
            println!("`next` - plays next track");
            println!("`prev` - plays prev track");
            println!("`ls` - current play queue items");
            println!("`ins [album/track url]` - add tracks from URL into play queue");
            println!("`del [queue index]` - remove track from play queue");
            println!("`seekf [secs]` - seek current track forward to `secs` seconds");
            println!("`seekb [secs]` - seek current track backward to `secs` seconds");
            println!("`voli [value]` - inscrease volume level");
            println!("`vold [value]` - decrease volume level");
            println!("`seek` [secs] - seek to absolute position on track");
        }
        _ => println!(
            "error: unknown command `{}` type `help` for commands",
            command_args[0]
        ),
    }
    Ok(())
}

pub fn load_interface(_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: running in cli mode");
    let mut queue = Queue::new();
    let mut search_results = Vec::new();

    loop {
        loop_control( &mut queue, &mut search_results)?;
    }
}

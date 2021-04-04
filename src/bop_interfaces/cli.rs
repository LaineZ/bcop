use std::{io::Write, time::Duration};

use crate::{
    bc_core::{self, playback::Player, queue::Queue},
    model::search,
};
use anyhow::Result;
use bc_core::{album_parsing::search, playback::FormatTime};

fn loop_control(
    player: &mut Player,
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
                        player.increase_volume(volume);
                        println!("volume set to: {}%", player.get_volume())
                    }
                    Err(_) => println!("error: invalid volume format"),
                }
            }
        }

        "vold" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u16>() {
                    Ok(volume) => {
                        player.decrease_volume(volume);
                        println!("volume set to: {}%", player.get_volume())
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
                        player.seek_forward(Duration::from_secs(seek));
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
                        player.seek(Duration::from_secs(seek));
                    }
                    Err(_) => println!("error: invalid seek format"),
                }
            }
        }

        "stop" => {
            player.stop();
            println!("player stopped!");
        }

        "seekb" => {
            if !command_args.is_empty() && command_args.len() > 1 {
                match command_args[1].parse::<u64>() {
                    Ok(seek) => {
                        println!("seeking backwards to {} seconds", seek);
                        player.seek_backward(Duration::from_secs(seek));
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
                if let Some(search_res) = search(command_args[1]) {
                    search_results.clear();
                    *search_results = search_res.results.clone();
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
                            &queue
                                .add_album_in_queue("Unknown artist".to_string(), url)
                                .unwrap();
                        } else {
                            println!("empty!");
                            log::info!("VALUES: {:#?}", search_results);
                        }
                    }
                    Err(_) => {
                        // load url
                        if command_args[1].starts_with("http") {
                            // TOOD: fix 'Unknown artist'
                            if queue
                                .add_album_in_queue("Unknown artist".to_string(), command_args[1])
                                .is_err()
                            {
                                println!("error while parsing album data!");
                            } else {
                                println!("track(s) inserted sucessfully type 'ls' to view it!");
                            }
                        }
                    }
                }
            }

            if !player.is_playing() {
                if let Some(track) = queue.get_current_track() {
                    player.switch_track(track.audio_url);
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
                            if let Some(track) = queue.get_current_track() {
                                player.switch_track(track.audio_url);
                                println!("starting playing...");
                            }
                        }
                        Err(_) => {
                            println!("incorrect format!");
                        }
                    }
                }
            } else {
                if player.is_paused() {
                    println!("info: playing");
                    player.play();
                } else {
                    println!("info: paused");
                    player.pause();
                }
            }
        }

        "d" => {
            if let Some(duration) = player.get_time() {
                println!("{}", FormatTime(duration));
            }
        }

        "next" => {
            if let Some(track) = queue.next() {
                player.switch_track(&track.audio_url);
                println!("next track: {}", &track);
            }
        }

        "prev" => {
            if let Some(track) = queue.prev() {
                player.switch_track(&track.audio_url);
                println!("switching to previous track: {}", &track);
            }
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
            println!("`seek` [secs] - seek to absolute position on track");
        }
        _ => println!(
            "error: unknown command `{}` type `help` for commands",
            command_args[0]
        ),
    }
    Ok(())
}

pub fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: running in cli mode");

    {
        let mut queue = Queue::new();
        let mut search_results = Vec::new();
        let mut player = Player::new();

        loop {
            loop_control(&mut player, &mut queue, &mut search_results)?;
        }
    }
}

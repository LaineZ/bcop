mod bop_core;
mod model;

use std::env;

use bop_core::playback;
use bytes::Bytes;
use model::album::Album;
use bop_core::get_tags;

fn loop_control(track_bytes: Bytes) {
    let device = rodio::default_output_device().unwrap();

    let mut sink = playback::create_sink(track_bytes.clone(), device, 0).unwrap();
    while !sink.empty() {
        let mut command = String::new();
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command_args: Vec<&str> = command.as_str().trim().split(' ').collect();
        match command_args[0] {
            "c" => std::process::exit(0),
            "exit" => std::process::exit(0),
            "vol" => {
                if !command_args.is_empty() && command_args.len() > 1 {
                    match command_args[1].parse::<f32>() {
                        Ok(volume) => {
                            sink.set_volume(volume);
                            println!("volume set at: {}", volume)
                        }
                        Err(_) => println!("error: invalid volume format"),
                    }
                }
            }

            "seek" => {
                if !command_args.is_empty() && command_args.len() > 1 {
                    match command_args[1].parse::<u32>() {
                        Ok(seek) => {
                            println!("seeking at: {}", seek);
                            let device = rodio::default_output_device().unwrap();
                            sink.stop();
                            sink =
                                playback::create_sink(track_bytes.clone(), device, seek).unwrap();
                        }
                        Err(_) => println!("error: invalid seek format"),
                    }
                }
            }

            "p" => {
                if sink.is_paused() {
                    println!("info: playing");
                    sink.play()
                } else {
                    println!("info: paused");
                    sink.pause()
                }
            }

            "next" => {
                println!("stopping current track");
                break;
            }

            "help" => {
                println!("command help:");
                println!("`c` - closes program\nALIAS: exit");
                println!("`p` - play/pause");
                println!("`next` - plays next track");
                println!("`vol [number: float]` - sets volume (default: 1.0) CAUTION: values above 1.0 causes a serious clipping!");
                println!("`seek [serconds: number]` - sets track position (in seconds)`")
            }
            _ => println!("error: unknown command `{}` type `help`", command_args[0]),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!(
        "BandcampOnlinePlayer by 140bpmdubstep version 0.1-alpha.rs\nCommand line: {:?}",
        args
    );

    if args.len() < 2 {
        eprintln!("error: Invalid number of arguments supplyed. Exiting");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "stream" => {
            println!("info: running in stream mode");
            let data = bop_core::get_album_data::get_tag_data(args[2].clone(), 5)
                .await
                .unwrap();
            for item in data.items {
                println!("loading album tracks: {} - {}", item.artist, item.title);
                let album_page: Result<String, reqwest::Error> =
                    bop_core::bop_http_tools::http_request(item.tralbum_url.as_str()).await;
                match album_page {
                    Ok(value) => {
                        let album_json = bop_core::get_album_data::get_album_data(value.as_str());
                        match album_json {
                            Some(album_value) => {
                                let album_json_fixed =
                                    bop_core::get_album_data::fix_json(album_value);
                                //println!("{}", album_json_fixed);
                                let data: Album =
                                    serde_json::from_str(album_json_fixed.as_str()).unwrap();
                                for track in data.trackinfo.unwrap() {
                                    println!("loading track: {}", track.title.unwrap());
                                    match track.file {
                                        Some(trackfile) => {
                                            let track_bytes = bop_core::playback::get_track_from_url(
                                                trackfile.mp3128.as_str(),
                                            )
                                            .await?;
                                            println!("playing: ready to accept commands type `help` to more info!");
                                            loop_control(track_bytes);
                                            println!("playback stopped");
                                        }
                                        None => {
                                            println!("warning: this cannot cannot be played because does not contain mp3 stream url!");
                                            continue;
                                        }
                                    }
                                }
                            }
                            None => println!("unable to start playback"),
                        }
                    }
                    Err(_) => {
                        println!("error: unconvertable error detected! Exiting...");
                        std::process::exit(1);
                    }
                }
            }
        }
        "streamtags" => {
            println!("available tags:");

            let tags = get_tags::get_tags().await?;
            for tag in tags {
                println!("{}", tag)
            }
        }
        _ => {
            eprintln!("error: Invalid arguments supplyed. Exiting");
            println!("Allowed options:");
            println!("stream [tag] - plays in commandline mode tracks from specified tag");
            println!("streamtags - show all most popular tags");
        }
    }
    Ok(())
}

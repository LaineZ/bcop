mod bop_core;
mod structs;
use std::env;
use std::thread;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // like a test

    let args: Vec<String> = env::args().collect();

    println!("BandcampOnlinePlayer by 140bpmdubstep version 0.1-alpha.rs\nCommand line: {:?}", args);
    
    if args.len() < 2 {
        eprintln!("error: Invalid number of arguments supplyed. Exiting");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "stream" => {
            println!("info: running in stream mode");
            let data: structs::struct_json_discover::Root = bop_core::get_album_data::get_tag_data(args[2].clone(), 1).await;
            for item in data.items {
                println!("loading album: {} - {}", item.artist, item.title);
                let album_page: Result<String, reqwest::Error> = bop_core::bop_http_tools::http_request(item.tralbum_url.as_str()).await;
                match album_page {
                    Ok(value) => {
                        let album_json = bop_core::get_album_data::get_album_data(value.as_str());
                        match album_json {
                            Some(album_value) => {
                                let album_json_fixed = bop_core::get_album_data::fix_json(album_value);
                                //println!("{}", album_json_fixed);
                                let data: structs::struct_json_album::Root = serde_json::from_str(album_json_fixed.as_str()).unwrap();
                                for track in data.trackinfo.unwrap() {
                                    let device = rodio::default_output_device().unwrap();
                                    println!("loading track: {}", track.title.unwrap());
                                    let track = bop_core::playback::get_track_from_url(track.file.mp3128.unwrap().as_str()).await;
                                    let sink = bop_core::playback::create_sink(track, device, 0);
                                    println!("playing: ready to accept commands type `help` to more info!");
                                    loop {
                                        if sink.empty() { break }
                                        let mut command = String::new();
                                        std::io::stdin().read_line(&mut command).expect("Failed to read line");
                                        let command_args: Vec<&str> = command
                                                                        .as_str()
                                                                        .trim()
                                                                        .split(" ")
                                                                        .collect();
                                        match command_args[0] {
                                            "c" => std::process::exit(0),
                                            "vol" => {
                                                if command_args.len() > 0 {
                                                    match command_args[1].parse::<f32>() {
                                                        Ok(volume) => {
                                                            sink.set_volume(volume);
                                                            println!("volume set at: {}", command_args[1])
                                                        }
                                                        Err(_) => println!("error: invalid volume format")
                                                    }
                                                }
                                            },
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
                                                println!("`c` - closes program");
                                                println!("`p` - play/pause");
                                                println!("`next` - plays next track");
                                                println!("`vol [number: float]` - sets volume (default: 1.0) CAUTION: values above 1.0 causes a serious clipping!");
                                            }
                                            _ => println!("error: unknown command `{}` type `help`", command_args[0])
                                        }
                                    }
                                }
                            }
                            None => println!("unable to start playback"),
                        }
                    }
                    Err(_) => {
                        panic!("unconvertable error detected! Exiting...");
                    }
                }
            }
        }
        _ => {
            eprintln!("error: Invalid arguments supplyed. Exiting");
            println!("Allowed options:\n[stream] [tag] - plays in commandline mode tracks from specified tag");
        }
    }
    Ok(())
}
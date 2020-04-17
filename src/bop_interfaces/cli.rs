use std::time::{Duration, Instant};

use crate::bop_core;
use crate::bop_core::album_parsing;
use crate::bop_core::playback;
use crate::bop_core::playback_advanced;
use crate::model::album;
use bytes::Bytes;
use anyhow::Result;

fn loop_control(track_bytes: Bytes) -> Result<()> {
    let device = rodio::default_output_device().unwrap();

    let mut sink = playback::create_sink(track_bytes.clone(), device, 0);

    let mut started_at = Instant::now();
    let mut paused_at: Option<Instant> = None;
    let mut pause_duration = Duration::from_secs(0);

    while !sink.empty() {
        let mut command = String::new();

        std::io::stdin().read_line(&mut command)?;

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
                            sink = playback::create_sink(track_bytes.clone(), device, seek);

                            started_at = Instant::now() - Duration::from_secs(seek.into());
                            pause_duration = Duration::from_secs(0);
                            paused_at = None;
                        }
                        Err(_) => println!("error: invalid seek format"),
                    }
                }
            }

            "p" => {
                if sink.is_paused() {
                    println!("info: playing");

                    if let Some(instant) = paused_at {
                        pause_duration += instant.elapsed();
                        paused_at = None;
                    }

                    sink.play()
                } else {
                    println!("info: paused");
                    paused_at = Some(Instant::now());
                    sink.pause()
                }
            }

            "d" => {
                let mut time = started_at.elapsed() - pause_duration;
                if let Some(paused_at) = paused_at {
                    time -= paused_at.elapsed();
                }

                let min = time.as_secs() / 60;
                let sec = time.as_secs() % 60;
                let ms = time.as_millis() % 1000;
                println!("{}:{:02}.{:03}", min, sec, ms);
            }

            "switchadvanced" => {
                println!("switching to advanced playback system!");
                let device = rodio::default_output_device().unwrap();
                sink.stop();
                sink = playback_advanced::create_sink(track_bytes.clone(), device, 0).unwrap();
            }

            "switchsimple" => {
                println!("switching to simple playback system!");
                let device = rodio::default_output_device().unwrap();
                sink.stop();
                sink = playback::create_sink(track_bytes.clone(), device, 0);
            }

            "next" => {
                println!("stopping current track");
                break;
            }

            "help" => {
                println!("command help:");
                println!("`c` - closes program\nALIAS: exit");
                println!("`p` - play/pause");
                println!("`d` - current track duration");
                println!("`next` - plays next track");
                println!("`vol [number: float]` - sets volume (default: 1.0) CAUTION: values above 1.0 causes a serious clipping!");
                println!("`seek [serconds: number]` - sets track position (in seconds)`")
            }
            _ => println!("error: unknown command `{}` type `help`", command_args[0]),
        }
    }
    Ok(())
}

pub async fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: running in stream mode");
    let data = bop_core::album_parsing::get_tag_data(args[2].clone(), 5)
        .await
        .unwrap();
    for item in data.items {
        println!("loading album tracks: {} - {}", item.artist, item.title);
        let album: Option<album::Album> = album_parsing::get_album(item.tralbum_url.as_str()).await;
        match album {
            Some(alb) => {
                for track in alb.trackinfo.unwrap() {
                    println!("loading track: {}", track.title.unwrap());
                    match track.file {
                        Some(trackfile) => {
                            let track_bytes =
                                bop_core::playback::get_track_from_url(trackfile.mp3128.as_str())
                                    .await?;
                            println!("playing: ready to accept commands type `help` to more info!");
                            loop_control(track_bytes)?;
                            println!("playback stopped");
                        }
                        None => {
                            println!("warning: this cannot cannot be played because does not contain mp3 stream url!");
                            continue;
                        }
                    }
                }
            }
            None => {
                println!("warning: encountered album error");
                continue;
            }
        }
    }
    Ok(())
}

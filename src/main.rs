mod bop_core;
mod structs;
use std::thread;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // like a test

    let args: Vec<String> = env::args().collect();

    println!("BandcampOnlinePlayer by 140bpmdubstep\nCommand line: {:?}", args);
    

    match args[1].as_str() {
        "stream" => {
            println!("Running in stream mode");
            let data: structs::struct_json_discover::Root = bop_core::get_album_data::get_tag_data(args[2].clone(), 1).await;
            for item in data.items {
                println!("Loading album: {} - {}", item.artist, item.title);
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
                                    println!("Playing track: {}", track.title.unwrap());
                                    let device = rodio::default_output_device().unwrap();
                                    let track = bop_core::playback::get_track_from_url(track.file.mp3128.unwrap().as_str()).await;
                                    let sink = bop_core::playback::create_sink(track, device, 0);
                                    sink.sleep_until_end();
                                }
                            }
                            None => println!("Unable to start playback"),
                        }
                    }
                    Err(_) => {
                        panic!("Unconvertable error detected! Exiting...");
                    }
                }
            }
        }
        _ => {
            println!("Allowed options:\n[stream] [tag(s)] - plays in commandline mode tracks from specified tag(s)");
            
        }
    }
    Ok(())
}
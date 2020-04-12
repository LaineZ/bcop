mod bop_core;
mod bop_interfaces;
mod model;

use std::env;

use bop_core::tags;
use bop_interfaces::cli;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!(
        "BandcampOnlinePlayer by 140bpmdubstep version 0.1-alpha.rs\nCommand line: {:?}",
        args
    );

    if args.len() < 2 {
        eprintln!("warning: no arguments supplyed exiting...");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "stream" => cli::cli_mode(args).await?,
        "streamtags" => {
            println!("available tags:");

            let tags = tags::get_tags().await?;
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

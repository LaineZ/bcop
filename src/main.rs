mod bop_core;
mod bop_interfaces;
mod model;

use std::env;

use bop_core::get_tags;
use bop_interfaces::cli;
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
        "stream" => cli::cli_mode(args).await?,
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

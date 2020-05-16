mod bc_core;
mod bop_interfaces;
mod model;

use std::env;
use log::LevelFilter;

use bc_core::tags;
use bop_interfaces::cli_advanced;
use log::{info, trace, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    simple_logging::log_to_file("bcrs.log", LevelFilter::Info)?;

    println!("BandcampOnlinePlayer by 140bpmdubstep version 0.3alpha.rs");

    info!("BandcampOnlinePlayer by 140bpmdubstep version 0.3alpha.rs Command line: {:?}", args);

    if args.len() < 2 {
        cli_advanced::loadinterface(args.clone())?;
        std::process::exit(0);
    }

    match args[1].as_str() {
        "cli" => cli_advanced::loadinterface(args)?,
        "streamtags" => {
            println!("available tags:");

            let tags = tags::get_tags()?;
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

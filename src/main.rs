mod bc_core;
mod bop_interfaces;
mod model;

use std::env;

use bc_core::tags;
use bop_interfaces::cli_advanced;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    flexi_logger::Logger::with_str("warn, bandcamp_online_cli=debug")
        .log_to_file()
        .format_for_files(flexi_logger::with_thread)
        .suppress_timestamp()
        .start()
        .unwrap();

    println!("BandcampOnlinePlayer by 140bpmdubstep and LeshaInc version 0.4alpha.rs");

    log::info!(
        "BandcampOnlinePlayer by 140bpmdubstep and LeshaInc version 0.4alpha.rs Command line: {:?}",
        args
    );

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

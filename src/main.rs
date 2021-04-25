mod bc_core;
mod bop_interfaces;
mod model;

//use bop_interfaces::cli;
use std::env;

use bop_interfaces::stream;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    flexi_logger::Logger::with_str("warn, bandcamp_online_cli=debug")
        .log_to_file()
        .format_for_files(flexi_logger::with_thread)
        .suppress_timestamp()
        .start()
        .unwrap();

    println!(
        "BandcampOnlinePlayer by 140bpmdubstep and LeshaInc VERSION {}",
        VERSION
    );

    log::info!(
        "BandcampOnlinePlayer by 140bpmdubstep and LeshaInc VERSION {} Command line: {:?}",
        VERSION,
        args
    );

    if args.len() < 2 {
        //cli::loadinterface(args.clone())?;
        std::process::exit(0);
    }

    match args[1].as_str() {
        //"tui" => tui::loadinterface(args)?,
        //"cli" => cli::loadinterface(args)?,
        "stream" => stream::loadinterface(args)?,
        _ => {
            eprintln!("error: Invalid arguments supplyed. Exiting");
        }
    }
    Ok(())
}

use std::time::Duration;

use crate::bc_core::{self, queue::Queue};
use anyhow::Result;
use bc_core::discover_loader::DiscoverLoader;

pub fn load_interface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: running in minimal-stream mode");
    let mut queue = Queue::new();
    let mut discover = DiscoverLoader::new();
    discover.extend_discover(args.clone())?;

    let mut idx = 0;
    if !discover.discover.is_empty() {
        queue.add_album_in_queue(discover.discover[idx].tralbum_url.as_str())?;
        queue.start();
    } else {
        println!("Cannot load discover, check your tags");
        std::process::exit(1);
    }

    loop {
        std::thread::sleep(Duration::from_millis(50));
        queue.process();
        if queue.is_end() {
            idx += 1;
            if idx >= discover.discover.len() - 1 {
            } else {
                discover.discover.clear();
                discover.extend_discover(args.clone())?;
                idx = 0;
            }

            queue.add_album_in_queue(discover.discover[idx].tralbum_url.as_str())?;
            queue.start();
        }
    }
}

use crate::{
  bc_core::{self, playback::Player, queue::Queue},
};
use anyhow::Result;
use bc_core::discover_loader::DiscoverLoader;
use std::time::Duration;



pub fn loadinterface(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("info: running in minimal-stream mode");
    let mut queue = Queue::new();
    let mut discover = DiscoverLoader::new();
    discover.extend_discover(args.clone())?;

    for album in &discover.discover {
      queue.add_album_in_queue(album.tralbum_url.as_str())?;
    }

    queue.start();

    loop {
    }
}

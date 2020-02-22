mod bop_core;
mod structs;
use tokio::prelude::*;

use futures::executor::block_on;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("получение тегов...");
    let tags = bop_core::get_tags::get_tags().await;
    println!("{:#?}", tags);
    println!("получение данных с тега 1...");
    let data = bop_core::get_album_data::get_tag_data(tags[1].clone(), 1).await;
    println!("{:#?}", data);
    Ok(())
}
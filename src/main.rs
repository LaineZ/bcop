mod bop_core;
mod structs;

use scraper::Html;
use scraper::Selector;
use std::vec::Vec;
use serde_json::{Value};
use regex::Regex;

fn main() {
    let response = bop_core::bop_http_tools::http_request("https://bandcamp.com/tags");
    let fragment = Html::parse_fragment(response.as_str());
    let selector = Selector::parse("a").unwrap();

    let mut tags = Vec::new();

    for element in fragment.select(&selector) {
        match element.value().attr("href") {
            None => {},
            Some(value) => {
                if value.starts_with("/tag/")
                {
                    //println!("{:?}", value.replace("/tag/", ""));
                    tags.push(value.replace("/tag/", ""));
                }
            }
        }
    }

    let client = reqwest::blocking::Client::new();
    for i in 1..5
    {
        println!("getting tag {}", tags[0]);
        let request_body = format!("{{\"filters\":{{ \"format\":\"all\",\"location\":0,\"sort\":\"pop\",\"tags\":[\"{}\"] }},\"page\":\"{}\"}}", tags[0], i);
        let res = client.post("https://bandcamp.com/api/hub/2/dig_deeper").body(request_body).send();
        match res {
            Ok(value) => {
                let v: structs::struct_json_discover::Root = serde_json::from_str(&value.text_with_charset("utf-8").unwrap()).unwrap();
                for item in v.items {
                    let html_code = &bop_core::bop_http_tools::http_request(&item.tralbum_url);
                    let tracks = bop_core::get_album_data::get_album_data(html_code);

                    match tracks {
                        Some(value) => {
                            
                        },
                        None => println!("failed to get: {}", &item.tralbum_url),
                    }
                    std::process::exit(0);
                }
             }
            Err(e) => { println!("Error: {}", e) }
        }
    }
}
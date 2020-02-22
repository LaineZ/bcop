use crate::bop_core;

use scraper::Html;
use scraper::Selector;
use std::vec::Vec;
use serde_json::{Value};


pub async fn get_tags() -> Vec<String> {
    let response = bop_core::bop_http_tools::http_request("https://bandcamp.com/tags").await;
    match response {
        Ok(value) => {
            let fragment = Html::parse_fragment(value.as_str());
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
            return tags;
        }
        Err(_) => {
            panic!("пиздец");
        }
    }
}
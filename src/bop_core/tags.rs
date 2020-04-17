#![allow(dead_code)]

use crate::bop_core;

use std::vec::Vec;

use anyhow::Result;
use scraper::Html;
use scraper::Selector;

pub async fn get_tags() -> Result<Vec<String>> {
    let response = bop_core::http_tools::http_request("https://bandcamp.com/tags").await?;
    let fragment = Html::parse_fragment(response.as_str());
    let selector = Selector::parse("a").unwrap();

    let mut tags: Vec<String> = fragment
        .select(&selector)
        .filter_map(|el| {
            let value = el.value().attr("href")?;
            if !value.starts_with("/tag/") {
                return None;
            }
            Some(value.replace("/tag/", ""))
        })
        .collect();
    
    tags.sort();
    tags.dedup();
    Ok(tags)
}

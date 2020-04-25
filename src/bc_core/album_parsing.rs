use crate::model::album::Album;
use crate::model::discover::DiscoverData;

use crate::bc_core::http_tools;

use anyhow::Result;
use regex::Regex;
use serde_json::json;

pub fn fix_json(data: &str) -> String {
    // fix url field
    let regex = Regex::new("(?P<root>url: \".+)\" \\+ \"(?P<album>.+\",)").unwrap();
    let data = regex.replace_all(data, "$root$album");

    // add quotes to fields
    let regex = Regex::new("    (?P<property>[a-zA-Z_]+):").unwrap();
    let data = regex.replace_all(&data, "\"$property\":");

    // remove comments
    let regex = Regex::new("// .*").unwrap();
    let data = regex.replace_all(&data, "");

    data.into()
}

pub async fn get_album(url: &str) -> Option<Album> {
    let page: Result<String, reqwest::Error> = http_tools::http_request(url).await;
    match page {
        Ok(value) => {
            let json = parse(value.as_str())?;
            let data: Album = serde_json::from_str(&json).unwrap();
            Some(data)
        }
        Err(_) => None,
    }
}

pub fn parse(html_code: &str) -> Option<String> {
    let start = "var TralbumData = {";
    let stop = "};";

    let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
    let album_data = &album_data[..=album_data.find(stop)?];
    let album_data_json = fix_json(album_data);
    Some(album_data_json)
}

pub async fn get_tag_data(tags: String, page: i32) -> Result<DiscoverData> {
    let client = reqwest::Client::new();

    let request = json!({
        "filters": {
            "format": "all",
            "location": 0,
            "sort": "pop",
            "tags": [tags]
        },
        "page": page
    });

    let response = client
        .post("https://bandcamp.com/api/hub/2/dig_deeper")
        .body(request.to_string())
        .send()
        .await?;

    let data = serde_json::from_str(response.text().await?.as_str())?;
    Ok(data)
}

use crate::model::discover::DiscoverData;

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

pub fn album_parsing(html_code: &str) -> Option<&str> {
    let start = "var TralbumData = {";
    let stop = "};";

    let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
    let album_data = &album_data[..=album_data.find(stop)?];
    Some(album_data)
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

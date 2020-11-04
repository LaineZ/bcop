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

pub fn get_album(url: &str) -> Option<Album> {
    let page = http_tools::http_request(url)?;
    let json = parse(page.as_str())?;
    let data: Album = serde_json::from_str(&json).unwrap();

    Some(data)
}

pub fn parse(html_code: &str) -> Option<String> {
    let start = "data-tralbum=\"{";
    let stop = "}\"";

    let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
    let album_data = &album_data[..=album_data.find(stop)?];
    let album_data_json = fix_json(&album_data.replace("&quot;", "\""));
    Some(album_data_json)
}

pub fn get_tag_data(tags: Vec<String>, page: usize) -> Result<DiscoverData> {
    let request = json!({
        "filters": {
            "format": "all",
            "location": 0,
            "sort": "pop",
            "tags": tags
        },
        "page": page
    });

    let response = ureq::post("https://bandcamp.com/api/hub/2/dig_deeper")
        .send_string(request.to_string().as_str());

    let data = serde_json::from_str(&response.into_string()?)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_album() {
        assert_eq!(
            crate::bc_core::album_parsing::get_tag_data(
                vec!["metal".to_string(), "death".to_string()],
                1
            )
            .unwrap()
            .ok,
            true
        )
    }
}

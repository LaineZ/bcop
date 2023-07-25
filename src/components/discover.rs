use base64::Engine;
use dioxus::prelude::*;
use scraper::{Html, Selector};
use serde_json::json;

use crate::{models::discover, services::config};

async fn set_image(url: String) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let bytes = response.bytes().await?;
    return Ok(format!(
        "data:image/jpeg;base64,{}",
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(bytes)
    ));
}

async fn get_tags_from_internet() -> anyhow::Result<Vec<String>> {
    log::info!("Loading tags from bandcamp.com...");
    let response = reqwest::get("https://bandcamp.com/api/hub/2/dig_deeper").await?;

    let resp = &response.text().await.unwrap_or_default();
    log::info!("{}", resp);

    let fragment = Html::parse_fragment(resp);
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
        .map(|f| {
            // capitalize tag letters
            let mut chars = f.chars();
            match chars.next() {
                Some(v) => v.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect();

    tags.sort();
    tags.dedup();
    // post processing
    tags.retain(|x| x.chars().all(char::is_alphanumeric) && !x.is_empty());

    Ok(tags)
}

async fn get_tags() -> Vec<String> {
    let file = std::fs::read_to_string("tag.cache");

    if let Ok(tags) = file {
        // use a cached tag file
        return tags.split("\n").map(|f| f.to_string()).collect();
    } else if let Ok(tags) = get_tags_from_internet().await {
        let tag_string = tags.join("\n");
        std::fs::write("tag.cache", tag_string.clone()).unwrap();
        return tags;
    }

    Vec::new()
}

async fn get_discover(tags: Vec<String>, page: usize) -> reqwest::Result<discover::Discover> {
    let request = json!({
        "filters": {
            "format": "all",
            "location": 0,
            "sort": "pop",
            "tags": tags
        },
        "page": page
    });

    let client = reqwest::Client::new();

    let response = client
        .post("https://bandcamp.com/api/hub/2/dig_deeper")
        .body(request.to_string())
        .send()
        .await?;

    response.json::<discover::Discover>().await
}
// components
#[derive(Props, PartialEq)]
pub struct DiscoverProps<'a> {
    discover: &'a Vec<discover::Item>,
    tags: String,
}

pub fn discover_list<'a>(cx: Scope<'a, DiscoverProps<'a>>) -> Element {
    cx.render(rsx!(div {
        h1 {
            "Select discover"
        }
        div {
        class: "discover-list",
            for discover in cx.props.discover.iter() {
                discover_item {
                    item: discover
                }
            }
        }
    }))
}

#[derive(Props, PartialEq)]
pub struct DiscoverItemProps<'a> {
    item: &'a discover::Item,
}

pub fn discover_item<'a>(cx: Scope<'a, DiscoverItemProps<'a>>) -> Element {
    let quality = config::ArtworkThumbnailQuality::Medium as u32;
    let art_id = cx.props.item.art_id;

    let image = use_state(cx, || String::new());

    use_future(cx, (), move |_| {
        let i = image.clone();
        async move {
            let img = set_image(format!(
                "http://f4.bcbits.com/img/a{}_{}.jpg",
                art_id, quality
            ))
            .await
            .unwrap_or_default();
            i.set(img);
        }
    });

    cx.render(rsx!(div {
        class: "album-card",
        img {
            class: "album-image",
            src: "{image.get()}",
        },
        div {
            class: "album-description",
            h4 {
                title: "{cx.props.item.title}",
                "{cx.props.item.title}"
            }
            p {
                "{cx.props.item.genre} × {cx.props.item.artist}"
            }
        }
    }))
}

pub fn discover_window(cx: Scope) -> Element {
    let tags = use_state(cx, || Vec::new());
    let discover = use_state(cx, || discover::Discover::default());
    let selected_tags = use_state(cx, || Vec::new());

    use_future(cx, (), move |_| {
        let t = tags.clone();
        async move {
            let tags = get_tags().await;
            t.set(tags);
        }
    });

    cx.render(rsx!(div {
        class: "discover",
        div {
            class: "tags-select",
            size: 5,
            for tag in tags.iter() {
                option {
                    dangerous_inner_html: "{tag}",
                    onclick: move |_| {
                        use_future(cx, (), move |_| {
                            let d = discover.clone();
                            let t = selected_tags.clone();
                            t.set(vec![tag.clone()]);
                            async move {
                                let dsc = get_discover(t.get().clone(), 1).await.unwrap();
                                d.set(dsc);
                            }
                        });
                    }
                }
            }
        },

        discover_list {
            discover: &discover.get().items,
            tags: selected_tags.get().join(" "),
        }
    }))
}

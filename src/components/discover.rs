use dioxus::prelude::*;
use scraper::{Html, Selector};

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
        return tags
    }

    Vec::new()
}

pub fn discover(cx: Scope) -> Element {
    let tags = use_state(cx, || Vec::new());

    use_future(cx, (), move |_| {
        let count = tags.clone();
        async move {
            let tags = get_tags().await;
            count.set(tags);
        }
    });

    cx.render(rsx!(div {
        class: "discover",
        div {
            class: "tags-select",
            size: 5,
            for tag in tags.iter() {
                option {
                    "{tag}"
                }
            }
        }
    }))
}

use std::time::Duration;

use regex::Regex;
use sciter::{dispatch_script_call, make_args, Element, Value};
use scraper::{Html, Selector};
use threadpool::ThreadPool;

const THREAD_COUNT: usize = 10;

pub struct HttpRequest {
    pool: ThreadPool,
}

fn fix_json(data: &str) -> String {
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

fn parse_album(html_code: String) -> Option<String> {
    let start = "data-tralbum=\"{";
    let stop = "}\"";

    let album_data = &html_code[html_code.find(start)? + start.len() - 1..];
    let album_data = &album_data[..=album_data.find(stop)?];
    let album_data_json = fix_json(&album_data.replace("&quot;", "\""));
    Some(album_data_json)
}

fn get_tags() -> anyhow::Result<Vec<String>> {
    let response = ureq::get("https://bandcamp.com/tags").call()?;
    let fragment = Html::parse_fragment(&response.into_string().unwrap_or(String::new()));
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

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            pool: ThreadPool::new(THREAD_COUNT),
        }
    }

    fn get_tags(&mut self, done: sciter::Value) {
        self.pool.execute(move || {
            let file = std::fs::read_to_string("tag.cache");

            if let Ok(tags) = file {
                // use a cached tag file
                done.call(None, &make_args!(tags), None).unwrap();
            }

            // if let Ok(tags) = get_tags() {
            //     // cache tag in file
            //     let tag_string = tags.join("\n");
            //     std::fs::write("tag.cache", tag_string.clone()).unwrap();
            //     done.call(None, &make_args!(tag_string.clone()), None)
            //         .unwrap();
            // }
        });
    }

    fn http_request_get(&mut self, url: String, done: sciter::Value) {
        self.pool.execute(move || match ureq::get(&url).call() {
            Ok(response) => {
                let body = response.into_string().unwrap();
                done.call(None, &make_args!(body), None).unwrap();
            }
            Err(err) => {
                log::error!("GET: Request to address: {} failed: {}", url, err);
            }
        });
    }

    fn http_request_post(&mut self, url: String, body: String, done: sciter::Value) {
        self.pool
            .execute(move || match ureq::post(&url).send_string(&body) {
                Ok(response) => {
                    let body = response.into_string().unwrap();
                    done.call(None, &make_args!(body), None).unwrap();
                }
                Err(err) => {
                    log::error!("POST: Request to address: {} failed: {}", url, err);
                }
            });
    }

    fn parse_album_data(&self, html_code: String) -> String {
        log::debug!("parsed");
        parse_album(html_code).unwrap_or(String::new())
    }

    fn open_in_browser(&self, url: String) -> bool {
        webbrowser::open(&url).is_ok()
    }

    fn set_image(&self, url: String, mut element: Element, proxy: bool) {
        self.pool.execute(move || {
            let url = if !proxy {
                url
            } else {
                format!(
                    "http://79.170.44.75/hostdoctordemo.co.uk/downloads/vpn/index.php?q={}&hl=3ed",
                    base64::encode(url)
                )
            };

            let response = ureq::get(&url).timeout(Duration::from_secs(30)).call();

            match response {
                Ok(resp) => {
                    if resp.status() == 200 {
                        let mut buf = Vec::new();
                        let _ = resp.into_reader().read_to_end(&mut buf);

                        element
                            .set_attribute(
                                "src",
                                &format!("data:image/jpeg;base64,{}", base64::encode(buf)),
                            )
                            .unwrap();
                    }
                }

                Err(msg) => {
                    log::error!("Failed to get artwork: {}", msg);
                }
            }
        });
    }
}

impl sciter::EventHandler for HttpRequest {
    dispatch_script_call! {
        fn http_request_get(String, Value);
        fn http_request_post(String, String, Value);
        fn set_image(String, Element, bool);
        fn parse_album_data(String);
        fn open_in_browser(String);
        fn get_tags(Value);
    }
}

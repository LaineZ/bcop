use std::time::Duration;

use base64::{engine::general_purpose, Engine};
use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use sciter::{dispatch_script_call, make_args, Element, Value};
use serde::Deserialize;
use threadpool::ThreadPool;
use ureq::Response;

const THREAD_COUNT: usize = 10;

pub struct HttpRequest {
    pool: ThreadPool,
    artwork_http: bool,
    request_http: bool,
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

#[derive(Deserialize)]
pub struct DiscoverAppData {
    #[serde(rename = "appData")]
    app_data: AppData,
}

#[derive(Deserialize)]
pub struct AppData {
    #[serde(rename = "initialState")]
    initial_state: InitialState,
}

#[derive(Deserialize)]
pub struct InitialState {
    genres: Vec<Data>,
    subgenres: Vec<Data>,
    locations: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Data {
    id: i32,
    label: String,
}

fn get_tags_from_internet() -> anyhow::Result<Vec<String>> {
    log::info!("Loading tags from bandcamp.com...");
    let response = ureq::get("https://bandcamp.com/discover").call()?;

    let response_text = &response.into_string().unwrap_or_default();
    log::info!("{}", response_text);

    let data;
    let data_regex =
        Regex::new(r#"<div[\n\s]+id="DiscoverApp"[\n\s]+data-blob="(?P<data>.+?)"[\n\s]*><"#)
            .unwrap();
    if let Some(captures) = data_regex.captures(response_text) {
        data = captures.name("data").unwrap().as_str().replace("&quot;", "\"");
    } else {
        return Err(anyhow::anyhow!("Can't find DiscoverApp json"));
    }

    let discover_app_data: DiscoverAppData = serde_json::from_str(&data).unwrap();
    let init_state = discover_app_data.app_data.initial_state;

    let mut tags: Vec<String> = init_state
        .genres
        .into_iter()
        .filter_map(|genre| {
            // filter 'all genres'
            if genre.id > 0 {
                Some(genre)
            } else {
                None
            }
        })
        .map(|genre| genre.label)
        .chain(
            init_state
                .subgenres
                .into_iter()
                .filter_map(|sub_genre| {
                    // filter parent genre tags (like 'all alternative')
                    if sub_genre.id >= 0 {
                        Some(sub_genre)
                    } else {
                        None
                    }
                })
                .map(|sub_genre| sub_genre.label),
        )
        .chain(
            init_state
                .locations
                .into_iter()
                .filter_map(|location| {
                    // filter 'from anywhere'
                    if location.id > 0 {
                        Some(location)
                    } else {
                        None
                    }
                })
                .map(|location| location.label),
        )
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

fn encode_response(
    resp: Result<Response, ureq::Error>,
    done: sciter::Value,
    failed: sciter::Value,
) {
    match resp {
        Ok(response) => {
            let body = response.into_string().unwrap_or_else(|op| {
                failed
                    .call(
                        None,
                        &make_args!(format!("Request reading failed: {}", op)),
                        None,
                    )
                    .unwrap();
                String::new()
            });
            //log::info!("{}", body);
            done.call(None, &make_args!(body), None).unwrap();
        }
        Err(err) => {
            failed
                .call(None, &make_args!(format!("Request failed: {}", err)), None)
                .unwrap();
            log::error!("Request failed: {}", err);
        }
    }
}

impl HttpRequest {
    pub fn new() -> Self {
        let artwork_http = ureq::head("https://f4.bcbits.com/")
            .timeout(Duration::from_secs(2))
            .call()
            .is_err();
        let request_http = ureq::head("https://bandcamp.com/")
            .timeout(Duration::from_secs(2))
            .call()
            .is_err();

        Self {
            pool: ThreadPool::new(THREAD_COUNT),
            artwork_http,
            request_http,
        }
    }

    fn artwork_http(&self) -> bool {
        self.artwork_http
    }

    fn request_http(&self) -> bool {
        self.request_http
    }

    fn get_tags(&self, done: sciter::Value) {
        log::info!("Active pool count: {}", self.pool.active_count());
        self.pool.execute(move || {
            let file = std::fs::read_to_string("tag.cache");

            if let Ok(tags) = file {
                // use a cached tag file
                done.call(None, &make_args!(tags), None).unwrap();
                return;
            }

            match get_tags_from_internet() {
                Ok(tags) => {
                    let tag_string = tags.join("\n");
                    std::fs::write("tag.cache", tag_string.clone()).unwrap();
                    done.call(None, &make_args!(tag_string), None).unwrap();
                }
                Err(error) => {
                    log::error!("{}", error.to_string());
                }
            }
        });
    }

    fn http_request_get(&self, url: String, done: sciter::Value, failed: sciter::Value) {
        self.pool.execute(move || {
            encode_response(
                ureq::get(&url).timeout(Duration::from_secs(3)).call(),
                done,
                failed,
            );
        });
    }

    fn http_request_post(
        &self,
        url: String,
        body: String,
        done: sciter::Value,
        failed: sciter::Value,
    ) {
        let proxy = ureq::Proxy::new("socks5://51.222.146.133:59166").unwrap();

        let agent = if self.request_http {
            ureq::AgentBuilder::new()
                .proxy(proxy)
                .timeout_connect(Duration::from_secs(5))
                .build()
        } else {
            ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(3))
                .build()
        };

        self.pool.execute(move || {
            encode_response(agent.post(&url).send_string(&body), done, failed);
        });
    }

    fn parse_album_data(&self, html_code: String) -> String {
        parse_album(html_code).unwrap_or_default()
    }

    fn open_in_browser(&self, url: String) -> bool {
        webbrowser::open(&url).is_ok()
    }

    fn copy_to_clipboard(&self, url: String) -> bool {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(url).is_ok()
    }

    fn set_image(&self, url: String, mut element: Element) {
        self.pool.execute(move || {
            let response = ureq::get(&url).timeout(Duration::from_secs(5)).call();

            match response {
                Ok(resp) => {
                    if resp.status() == 200 {
                        let mut buf = Vec::new();
                        let _ = resp.into_reader().read_to_end(&mut buf);

                        element
                            .set_attribute(
                                "src",
                                &format!(
                                    "data:image/jpeg;base64,{}",
                                    general_purpose::STANDARD_NO_PAD.encode(buf)
                                ),
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

impl Default for HttpRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl sciter::EventHandler for HttpRequest {
    dispatch_script_call! {
        fn http_request_get(String, Value, Value);
        fn http_request_post(String, String, Value, Value);
        fn set_image(String, Element);
        fn parse_album_data(String);
        fn open_in_browser(String);
        fn copy_to_clipboard(String);
        fn get_tags(Value);
        fn artwork_http();
        fn request_http();
    }
}

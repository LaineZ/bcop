use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::LogicalPosition, wry::http::Response, LogicalSize, WindowBuilder};
use dioxus_router::{Link, Route, Router};
use load_file::load_bytes;
use reqwest::header::CONTENT_TYPE;

pub mod icons;

pub mod components;
//pub mod handlers;
pub mod models;
pub mod players;
pub mod services;
pub mod url_resolver;

#[cfg(target_os = "windows")]
fn hide_console_window() {
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if !window.is_null() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn hide_console_window() {
    // just do nothing
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hide_console_window();
    env_logger::init();

    let configuration = services::config::Config::new();

    let wb = WindowBuilder::new()
        .with_focused(true)
        .with_title("BandcampOnlinePlayer")
        .with_resizable(true)
        .with_position(LogicalPosition::new(
            configuration.window_geometry.x,
            configuration.window_geometry.y,
        ))
        .with_inner_size(LogicalSize::new(
            configuration.window_geometry.w,
            configuration.window_geometry.h,
        ));

    let desktop_config = dioxus_desktop::Config::default()
        .with_window(wb)
        .with_custom_head(include_str!("assets/head.html").to_string())
        .with_custom_protocol("assets".into(), |request|  {
            let asset_path = url_resolver::resolve_assets_url(request);

            let mime = mime_guess::from_path(asset_path.clone())
                .first_raw()
                .unwrap_or("");

            println!("{} -> {}", request.uri(), asset_path.to_str().unwrap());

            let file_conetent: &[u8] = match asset_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
            {
                "style.css" => {
                    include_bytes!("assets/style.css")
                },
                "themes.js" => {
                    include_bytes!("assets/themes.js")
                },
                "opensans.ttf" => {
                    include_bytes!("assets/opensans.ttf")
                }
                _ => {
                    &[]
                }

            };

            Response::builder()
                .header(CONTENT_TYPE, mime)
                .header("Access-Control-Allow-Origin", "*")
                .body(file_conetent.into())
                .map_err(Into::into)
        });
    dioxus_desktop::launch_cfg(app, desktop_config);

    Ok(())
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            main {
                div {
                    class: "main-window",
                    menu {
                        Link { to: "/", icons::home(cx), },
                        Link { to: "/now-playing", icons::loading(cx), },
                        Link { to: "/tags", icons::tags(cx), },
                        icons::settings(cx),
                        icons::search(cx),
                    }

                    div {
                        Route { to: "/", components::home::home(cx)}
                        Route { to: "/now-playing", "Now Playing" }
                        Route { to: "/tags", components::discover::discover_window(cx)}
                    }
                }

                div {
                    class: "player-controls",
                    button {
                        icons::previous(cx)
                    }
                    button {
                        icons::play(cx),
                    }
                    button {
                        icons::next(cx),
                    }
                    div {
                        class: "track-information",
                        components::seekbar::seekbar {
                            min: 0,
                            max: 100,
                            on_value_change: move |event| {
                                println!("track:{}", event);
                            }
                        },
                        div {
                            class: "track-bottom",
                            p {
                                id: "track-name",
                                "No track loaded"
                            }
                            div {
                                id: "track-clock",
                                components::seekbar::seekbar {
                                    min: 0,
                                    max: 100,
                                    on_value_change: move |event| {
                                        println!("{}", event);
                                    }
                                },
                                p {
                                    dangerous_inner_html: "00:00 <strong>00:00</strong>"
                                },
                            }
                        }
                    }
                }
            }
        }
    ))
}

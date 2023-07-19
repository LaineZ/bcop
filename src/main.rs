use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::LogicalPosition, wry::http::Response, LogicalSize, WindowBuilder};
use dioxus_router::{Link, Route, Router};
use load_file::load_bytes;
use reqwest::header::CONTENT_TYPE;

pub mod icons;

pub mod components;
pub mod handlers;
pub mod models;
pub mod players;
pub mod services;

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
        .with_custom_protocol("assets".into(), |request| {
            let path = request
                .uri()
                .path()
                .to_string()
                .chars()
                .into_iter()
                .skip(1)
                .collect::<String>();

            let mut asset_path = PathBuf::new();
            asset_path.push("assets/");
            asset_path.push(path.clone());

            let mime = mime_guess::from_path(asset_path.clone())
                .first_raw()
                .unwrap_or("");

            println!(
                "{}: {} -> {}",
                request.uri(),
                path,
                asset_path.to_str().unwrap()
            );

            if path != "/" {
                Response::builder()
                    .header(CONTENT_TYPE, mime)
                    .body(load_bytes!(asset_path.to_str().unwrap()).into())
                    .map_err(Into::into)
            } else {
                Response::builder().body("".into()).map_err(Into::into)
            }
        });
    dioxus_desktop::launch_cfg(app, desktop_config);

    Ok(())
}

fn app(cx: Scope) -> Element {
    let current_pos = use_state(cx, || 0);

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
                        Route { to: "/tags", components::discover::discover(cx)}
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
                        div {
                            class: "seekbar",
                            style: "background: linear-gradient(
                                to right, 
                                var(--fg),
                                var(--fg) {current_pos}%,
                                var(--bg1) {current_pos}%,
                                var(--bg1)
                              );",
                            input {
                                r#type: "range",
                                min: 0,
                                max: 320,
                                value: 0,
                                oninput: move |e| {
                                    current_pos.set((e.value.parse::<f32>().unwrap_or_default() / 320.0 * 100.0) as i32);
                                },
                                onchange: move |e| {
                                    current_pos.set((e.value.parse::<f32>().unwrap_or_default() / 320.0 * 100.0) as i32);
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

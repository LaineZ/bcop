use std::sync::Arc;

use tokio::sync::Mutex;

use anyhow::anyhow;
use sciter::Value;

pub mod handlers;
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

fn check_options() {
    for arg in std::env::args() {
        if arg.starts_with("--sciter-gfx=") {
            use sciter::GFX_LAYER;
            let backend = match arg.split_at("--sciter-gfx=".len()).1.trim() {
                "auto" => GFX_LAYER::AUTO,
                "cpu" => GFX_LAYER::CPU,
                "skia" | "skia-cpu" => GFX_LAYER::SKIA_CPU,
                "skia-opengl" => GFX_LAYER::SKIA_OPENGL,

                #[cfg(windows)]
                "d2d" => GFX_LAYER::D2D,
                #[cfg(windows)]
                "warp" => GFX_LAYER::WARP,

                _ => GFX_LAYER::AUTO,
            };
            log::info!("setting {:?} backend", backend);
            let ok = sciter::set_options(sciter::RuntimeOptions::GfxLayer(backend));
            if let Err(e) = ok {
                log::error!("failed to set backend: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hide_console_window();
    env_logger::init();
    check_options();

    let config = services::config::Config::new();
    let mut player = services::player::Player::new(config.audio_system, config.device_index);

    let mut frame = sciter::WindowBuilder::main_window()
        .with_rect(config.window_geometry.into())
        .create();

    frame
        .set_options(sciter::window::Options::DebugMode(true))
        .unwrap();
    frame.event_handler(handlers::http_request::HttpRequest::new());
    frame.event_handler(handlers::log::Log);
    //frame.event_handler(config);
    frame.event_handler(handlers::io::Io);
    frame.event_handler(handlers::player::Player::new(&mut player));

    frame.set_variable("debugMode", Value::from(cfg!(debug_assertions)))?;
    frame.set_variable("bcRsVersion", Value::from(env!("CARGO_PKG_VERSION")))?;

    if cfg!(debug_assertions) {
        let dir = std::env::current_dir()?.join("frontend");

        if dir.exists() {
            frame.load_file(dir.join("index.html").to_str().unwrap());
        } else {
            return Err(
                anyhow!("Unable to find {} directory. You running in debug mode, you need fronend/ directory in bc_rs
                working directory. If you don't want that please build in release mode.", 
                dir.display()));
        }
    } else {
        let resources = include_bytes!("archive.rc");
        frame.archive_handler(resources).map_err(|_| {
            anyhow!("Invalid archive, cannot load.")
        })?;
        frame.load_file("this://app/index.html");
    }

    frame.run_app();

    Ok(())
}

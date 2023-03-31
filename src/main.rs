use anyhow::anyhow;
use handlers::config::AudioSystem;
use players::{bass::BassPlayer, internal::InternalPlayer};
use sciter::Value;

pub mod handlers;
pub mod players;

#[cfg(target_os = "windows")]
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
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

fn main() -> anyhow::Result<()> {
    hide_console_window();
    env_logger::init();
    check_options();

    let config = handlers::config::Config::new();

    let mut frame = sciter::WindowBuilder::main_window()
        .with_rect(config.window_geometry.into())
        .create();

    let player_handler = match config.get_audio_system() {
        AudioSystem::Internal => handlers::player::Player::new(Box::new(InternalPlayer::new())),
        AudioSystem::Bass => {
            if let Ok(pl) = BassPlayer::new() {
                handlers::player::Player::new(Box::new(pl))
            } else {
                log::error!("BASS library initialization failed: falling back to default player implementation");
                handlers::player::Player::new(Box::new(InternalPlayer::new()))
            }
        }
    };

    frame
        .set_options(sciter::window::Options::DebugMode(true))
        .unwrap();
    frame.event_handler(handlers::http_request::HttpRequest::new());
    frame.event_handler(handlers::log::Log);
    frame.event_handler(config);
    frame.event_handler(handlers::io::Io);
    frame.event_handler(player_handler);
    frame.set_variable("debugMode", Value::from(cfg!(debug_assertions)))?;

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
        frame.archive_handler(resources).expect("Invalid archive");
        frame.load_file("this://app/index.html");
    }

    frame.run_app();

    Ok(())
}

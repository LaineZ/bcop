use anyhow::anyhow;
use rsciter::{app, Visibility, Window, WindowFlags};
//pub mod handlers;
//pub mod players;

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

fn main() -> anyhow::Result<()> {
    hide_console_window();
    env_logger::init();
    app::init()?;

    let _v = rsciter::setup_debug_output(|sub, sev, text| {
        match sev {
            rsciter::DebugSeverity::Info => {
                log::info!("{:?}: {text}", sub);
            },
            rsciter::DebugSeverity::Warning => {
                log::info!("{:?}: {text}", sub);
            },
            rsciter::DebugSeverity::Error => {
                log::error!("{:?}: {text}", sub);
            },
            rsciter::DebugSeverity::Unknown(_) => {
                unimplemented!()
            },
        }
    })?;

    //let config = handlers::config::Config::new();
    //let audio_system = config.get_audio_system();
    //let audio_index = config.get_audio_device_index();
    let window = Window::builder().with_flags(WindowFlags::HAS_TITLEBAR | WindowFlags::HAS_CONTROLS | WindowFlags::RESIZEABLE);

    if cfg!(debug_assertions) {
        let dir = std::env::current_dir()?.join("frontend");

        if dir.exists() {
            let window = window.with_file(dir.join("index.html").to_str().unwrap()).build_main()?;
            window.show(Visibility::Normal)?;
        } else {
            return Err(
                anyhow!("Unable to find {} directory. You running in debug mode, you need fronend/ directory in bc_rs
                working directory. If you don't want that please build in release mode.", 
                dir.display()));
        }
    } else {
        let resources = include_bytes!("archive.rc");
        let window = window.with_archive_static(resources)
        .with_file("this://app/main.html")
        .build_main()?;
        window.show(Visibility::Normal)?;
    }

    app::run()?;

    Ok(())
}

pub mod handlers;
pub mod playback;

fn check_options() {
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`
		| sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8, // Enables opening file dialog (`view.selectFile()`)
    ))
    .ok();

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
    env_logger::init();
    check_options();

    log::info!("sciter: {}", sciter::version());
    let resources = include_bytes!("archive.rc");

    let mut frame = sciter::WindowBuilder::main_window()
        .with_size((1000, 600))
        .create();

    frame.archive_handler(resources).expect("Invalid archive");
    frame
        .set_options(sciter::window::Options::DebugMode(true))
        .unwrap();
    frame.event_handler(handlers::log::Log);
    frame.event_handler(handlers::http_request::HttpRequest::new());
    frame.event_handler(handlers::player::Player::new());
    frame.event_handler(handlers::config::Config::new());
    frame.load_file("this://app/index.html");
    frame.run_app();
    Ok(())
}

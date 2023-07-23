use std::path::PathBuf;

use dioxus_desktop::wry::http::Request;

//#[cfg(not(target_os = "windows"))]
pub fn resolve_assets_url(request: &Request<Vec<u8>>) -> PathBuf {
    let path = request
        .uri()
        .to_string()
        .replace("assets://", "")
        .chars()
        .into_iter()
        .collect::<String>();

    println!("{}", path);

    let mut asset_path = PathBuf::new();
    asset_path.push("assets/");
    asset_path.push(path.clone());
    asset_path
}
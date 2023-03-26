use std::{
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

fn main() {
    let sdk_path = Path::new("sciter-js-sdk-4.4.9.3");
    let mut out_dir = PathBuf::from_str(&std::env::var("OUT_DIR").unwrap()).unwrap();

    let platform = std::env::var("TARGET").unwrap();

    //println!("cargo:warning={}", platform);
    for _ in 0..3 {
        out_dir.pop();
    }

    //println!("cargo:warning=path: {:?}", out_dir);

    if !sdk_path.exists() {
        if cfg!(windows) {
            panic!("Unable to find sciter sdk installation... please run ./download.ps1 script first!\n
            If script fails to run please type in powershell:
            PS > Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope CurrentUser
            ");
        } else {
            panic!("Unable to find sciter sdk installation... please run ./download.sh script first!");
        }
    }

    if platform.starts_with("x86_64-pc-windows") {
        std::fs::copy(
            sdk_path.join("bin/windows/x64/sciter.dll"),
            out_dir.join("sciter.dll"),
        )
        .unwrap();
    } else if platform.starts_with("aarch64-pc-windows") {
        std::fs::copy(
            sdk_path.join("bin/windows/arm64/sciter.dll"),
            out_dir.join("sciter.dll"),
        )
        .unwrap();
    } else if platform.starts_with("i686-pc-windows") {
        std::fs::copy(
            sdk_path.join("bin/windows/x32/sciter.dll"),
            out_dir.join("sciter.dll"),
        )
        .unwrap();
    } else if platform.starts_with("x86_64-unknown-linux") {
        std::fs::copy(
            sdk_path.join("bin/linux/x64/libsciter-gtk.so"),
            out_dir.join("libsciter-gtk.so"),
        )
        .unwrap();
    } else if platform.starts_with("aarch64-unknown-linux") {
        std::fs::copy(
            sdk_path.join("bin/linux/arm64/libsciter-gtk.so"),
            out_dir.join("libsciter-gtk.so"),
        )
        .unwrap();
    } else if platform.starts_with("armv7-unknown-linux") {
        std::fs::copy(
            sdk_path.join("bin/linux/arm32/libsciter-gtk.so"),
            out_dir.join("libsciter-gtk.so"),
        )
        .unwrap();
    } else if platform == "x86_64-apple-darwin" {
        std::fs::copy(
            sdk_path.join("bin/macosx/libsciter.dylib"),
            out_dir.join("libsciter.dylib"),
        )
        .unwrap();
    } else {
        panic!("{} is not supported target by sciter", platform);
    }

    let path = if cfg!(windows) {
        "bin/windows"
    } else if cfg!(unix) {
        "bin/linux"
    } else if cfg!(target_os = "macos") {
        "bin/macosx"
    } else {
        panic!("Build platform does not support sciter `packfolder` executable!");
    };

    Command::new(sdk_path.join(path).join("packfolder"))
        .args(["frontend/", "src/archive.rc", "-binary"])
        .status()
        .expect("Unable to execute `packfolder` tool");

    #[cfg(windows)]
    {
        use winres::WindowsResource;
        WindowsResource::new()
            .set_icon("frontend/icons/icon.ico")
            .set_language(0x0009)
            .set("LegalCopyright", "© 140bpmdubstep")
            .set("ProductName", "BandcampOnlinePlayer")
            .set("FileDescription", 
            "Simple and user-friendly desktop-oriented client for Bandcamp.com to listen to albums from tags or URLs.")
            .compile().unwrap();
    }
}

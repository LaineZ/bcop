use std::{path::Path, process::Command};

fn main() {
    let sdk_path = Path::new("sciter-js-sdk-4.4.9.3");

    if !sdk_path.exists() {
        panic!("Unable to find sciter sdk installation... please run ./download.sh script...");
    }

    let var = &std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(var);


    if cfg!(windows) {

        match std::env::consts::ARCH {
            "x86_64" => {
                std::fs::copy(sdk_path.join("bin/windows/x64/sciter.dll"), out_dir.join("sciter.dll")).unwrap();
            }

            "x86" => {
                std::fs::copy(sdk_path.join("bin/windows/x32/sciter.dll"), out_dir.join("sciter.dll")).unwrap();
            }

            "aarch64" => {
                std::fs::copy(sdk_path.join("bin/windows/arm64/sciter.dll"), out_dir.join("sciter.dll")).unwrap();
            }

            v => {
                panic!("Architecture not supported: {}", v)
            }
        }
    } else if cfg!(unix) {
        match std::env::consts::ARCH {
            "x86_64" => {
                println!("{:?} -> {:?}", sdk_path.join("bin/linux/x64/libsciter-gtk.so"), out_dir.join("libsciter-gtk.so"));
                std::fs::copy(sdk_path.join("bin/linux/x64/libsciter-gtk.so"), out_dir.join("libsciter-gtk.so")).unwrap(); 
            }

            "aarch64" => {
                std::fs::copy(sdk_path.join("bin/linux/arm64/libsciter-gtk.so"), out_dir.join("libsciter-gtk.so")).unwrap(); 
            }

            "arm" => {
                std::fs::copy(sdk_path.join("bin/linux/arm32/libsciter-gtk.so"), out_dir.join("libsciter-gtk.so")).unwrap(); 
            }

            v => {
                panic!("Architecture not supported: {}", v)
            }
        }
    } else if cfg!(target_os = "macos") {
        std::fs::copy(sdk_path.join("bin/macosx/libsciter.dylib"), out_dir.join("libsciter.dylib")).unwrap(); 
    } else {
        panic!("OS is not supported right now...");
    }

    Command::new(sdk_path.join("bin/linux/packfolder"))
        .args(&["frontend/", "src/archive.rc", "-binary"])
        .status()
        .expect("Unable to find `packfolder` tool in $PATH");
}

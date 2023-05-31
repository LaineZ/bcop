use std::path::Path;

use bass_rs::{*};
mod device_tests;
mod samplechannel_tests;
mod streamchannel_tests;

macro_rules! __bass_check {
    ($res:expr) => {
        match $res {
            Err(e) => panic!("Bass error: {:?}", e),
            Ok(e) => e
        }
    };
}

#[test]
pub fn test() {
    // copy the bass lib file into the exec dir
    move_dll();


    // init bass
    let bass = __bass_check!(Bass::builder().build());

    // check test.mp3 exists before testing channels
    if !Path::new("./test.mp3").exists() {
        println!("test.mp3 missing, not running channel tests");
        return;
    }

    // device tests
    // __bass_check!(device_tests::device_tests());

    // channel tests

    __bass_check!(samplechannel_tests::sample_channel_tests());

    __bass_check!(streamchannel_tests::stream_channel_tests());


    drop(bass);
}

fn move_dll() {
    #[cfg(target_os = "windows")]
    let filename = "bass.dll";
       
    #[cfg(target_os = "linux")]
    let filename = "libbass.so";

    #[cfg(target_os = "macos")]
    let filename = "libbass.dylib";

    if let Ok(mut library_path) = std::env::current_exe() {
        library_path.pop();
        library_path.push(filename);

        println!("lib dir: {:?}", library_path);
        if library_path.exists() {return}

        std::fs::copy(filename, library_path).expect("error copying lib to exe path");
    } else {
        panic!("error with current dir for lib")
    }
}

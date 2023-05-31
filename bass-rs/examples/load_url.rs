use bass_rs::{Bass, prelude::StreamChannel};

fn main() {
    let _bass = Bass::builder().build().expect("Unable to initialize BASS!");
    let stream = StreamChannel::load_from_url("https://dj.ru/user_music/tracks/420566.mp3", 0).unwrap();
    stream.play(false).unwrap();

    println!("Loading track...");

    loop {

    }

}
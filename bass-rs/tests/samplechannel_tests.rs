use std::env::current_dir;

use bass_rs::prelude::*;

pub fn sample_channel_tests() -> BassResult<()> {
    let current_dir = current_dir().unwrap();
    let file_path = current_dir.join("test.mp3");

    // read test file
    let stream = {
        let file_path = file_path.to_str().unwrap();
        // create stream
        let stream = SampleChannel::load_from_path(file_path, 0, 32)?;
        stream
    };

    // stream.set_attribute(ChannelAttribute::MusicSpeed, 2.0)?;

    // try playing
    stream.play(false)?;

    stream.set_position(20.0)?;
    stream.set_volume(0.1)?;
    stream.set_attribute(ChannelAttribute::Pan, -1.0)?;

    let current_freq = stream.get_attribute(ChannelAttribute::Frequency)?;
    stream.set_attribute(ChannelAttribute::Frequency, current_freq * 1.7)?;
    stream.get_length_seconds()?;

    Ok(())
}
use bass_rs::prelude::*;

pub fn stream_channel_tests() -> BassResult<()> {

    // read test file
    let stream = {
        let file_path = "./test.mp3";
        let bytes = std::fs::read(file_path).expect("Error reading ./test.mp3");

        // create stream
        let stream = StreamChannel::load_from_memory(bytes, 0)?;
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

    Ok(())
}
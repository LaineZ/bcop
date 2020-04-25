use crate::bc_core::http_tools;
use anyhow::Result;
use bytes::Bytes;
use rodio::Source;
use std::io::Cursor;

use minimp3::{Error, Frame};

pub async fn get_track_from_url(url: &str) -> Result<Bytes> {
    let bytes = http_tools::http_request_bytes(url).await?;
    Ok(bytes)
}

pub fn create_sink(bytes: Bytes, device: rodio::Device, seek_sec: u32) -> rodio::Sink {
    let cursor = Cursor::new(bytes.clone());

    // lol
    let mut decoder_for_duration = minimp3::Decoder::new(cursor);

    let mut duration: f32 = 0.0;
    loop {
        match decoder_for_duration.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => {
                let lendata = data.len() as f32;
                duration += (lendata / channels as f32) / sample_rate as f32;
                //println!("sample size: {}", (lendata / channels as f32) / sample_rate as f32);
            }
            Err(Error::Eof) => break,
            Err(e) => println!("error: decoding sample failed: {:?}", e),
        }
    }

    let sink = rodio::Sink::new(&device);
    let cursor = Cursor::new(bytes.clone());
    let mut decoder = rodio::Decoder::new(cursor).unwrap();

    if duration as u32 >= seek_sec {
        for _ in 0..(seek_sec * decoder.sample_rate()) {
            decoder.next();
        }
    } else {
        println!("warining: ignoring seek larger than {} secs", duration);
    }
    sink.append(decoder);
    sink
}

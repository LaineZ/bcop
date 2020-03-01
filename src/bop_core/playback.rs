use std::io::Cursor;

use anyhow::Result;
use bytes::Bytes;
use rodio::buffer::SamplesBuffer;
use rodio::{Decoder, Sink, Source};

use crate::bop_core::bop_http_tools;

pub async fn get_track_from_url(url: &str) -> Result<Bytes> {
    let bytes = bop_http_tools::http_request_bytes(url).await?;
    Ok(bytes)
}

pub fn create_sink(bytes: Bytes, device: rodio::Device, seek_sec: u32) -> Result<Sink> {
    let cursor = Cursor::new(bytes);

    let decoder = Decoder::new(cursor)?;
    let channels = decoder.channels();
    let sample_rate = decoder.sample_rate();

    let samples: Vec<_> = decoder.collect();
    let mut buffer = SamplesBuffer::new(channels, sample_rate, samples);

    let sink = Sink::new(&device);

    let total_dur = buffer.total_duration().unwrap().as_secs();

    if u64::from(seek_sec) < total_dur {
        for _ in 0..(seek_sec * buffer.sample_rate()) {
            buffer.next();
        }
    } else {
        println!("warning: ignoring seeking larger than {} secs", total_dur)
    }

    sink.append(buffer);
    Ok(sink)
}

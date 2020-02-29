use crate::bop_core::bop_http_tools;
use anyhow::Result;
use bytes::Bytes;
use rodio::Source;
use std::io::Cursor;



pub async fn get_track_from_url(url: &str) -> Result<Bytes> {
    let bytes = bop_http_tools::http_request_bytes(url).await?;
    Ok(bytes)
}

pub fn create_sink(bytes: Bytes, device: rodio::Device, seek_sec: u32) -> rodio::Sink {
    let cursor = Cursor::new(bytes);
    
    let sink = rodio::Sink::new(&device);

    let mut decoder = rodio::Decoder::new(cursor).unwrap();

    match decoder.total_duration() {
        Some(dur) => {
            let total_dur: u32 = dur.as_secs() as u32;
            if seek_sec < total_dur {
            for _ in 0..(seek_sec * decoder.sample_rate()) { decoder.next(); }
            } else {
                println!("warning: ignoring seeking larger than {} secs", total_dur)
            }
        }
        None => {
            println!("warning: unable to determine duration for this track, seek may crash program");
            for _ in 0..(seek_sec * decoder.sample_rate()) { decoder.next(); }
        }
    }
    sink.append(decoder);
    sink
}
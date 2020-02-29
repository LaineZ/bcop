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
    for _ in 0..(seek_sec * decoder.sample_rate()) { decoder.next(); }
    sink.append(decoder);
    sink
}
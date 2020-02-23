use crate::bop_core::bop_http_tools;
use bytes::Bytes;
use rodio::Source;
use std::io::Cursor;

pub async fn get_track_from_url(url: &str) -> Cursor<Bytes> {
    let file_bytes = bop_http_tools::http_request_bytes(url).await;
    match file_bytes {
        Ok(url_bytes) => {
            Cursor::new(url_bytes)
        }
        Err(_) => {
            panic!("Error");
        }
    }
}

pub fn create_sink(cursor: Cursor<Bytes>, device: rodio::Device, seek_sec: u32) -> rodio::Sink {
    let sink = rodio::Sink::new(&device);

    let mut decoder = rodio::Decoder::new(cursor).unwrap();
    for _ in 0..(seek_sec * decoder.sample_rate()) { decoder.next(); }
    sink.append(decoder);
    sink
}
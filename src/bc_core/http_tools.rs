use bytes::Bytes;
use std::io::Read;

pub fn http_request(url: &str) -> Option<String> {
    let response = ureq::get(url).call();

    //println!("info: status: {}", res.status());

    let body = response.into_string().unwrap();

    Some(body)
}

pub fn http_request_bytes(url: &str) -> Option<Bytes> {
    let response = ureq::get(url).call();

    //println!("info: status: {}", res.status());


    let mut reader = response.into_reader();
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes);


    let bytes = Bytes::from(bytes);

    Some(bytes)
}

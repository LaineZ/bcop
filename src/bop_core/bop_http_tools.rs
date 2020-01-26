pub fn http_request(address: &str) -> String {
        let body = reqwest::blocking::get(address)
        .unwrap()
        .text_with_charset("utf-8");

        match body {
            Ok(v) => return v,
            Err(e) => panic!("error: {:?}", e),
        }
    }
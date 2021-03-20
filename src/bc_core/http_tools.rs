pub fn http_request(url: &str) -> Option<String> {
    let response = ureq::get(url).call().expect("Unable to create web request, check your internet");
    //println!("info: status: {}", res.status());
    let body = response.into_string().unwrap();

    Some(body)
}

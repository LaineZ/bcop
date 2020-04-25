use bytes::Bytes;

pub async fn http_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    //println!("info: status: {}", res.status());

    let body = res.text().await?;

    Ok(body)
}

pub async fn http_request_bytes(url: &str) -> Result<Bytes, reqwest::Error> {
    let res = reqwest::get(url).await?;

    //println!("info: status: {}", res.status());

    let body = res.bytes().await?;

    Ok(body)
}

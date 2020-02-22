pub async fn http_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    Ok(body)
}

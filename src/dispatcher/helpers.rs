use reqwest;
use serde_json;
use crate::configuration;

pub async fn make_the_get(headers:reqwest::header::HeaderMap, url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true) // Disable SSL verification
    .build()?;

    let response = client
        .get(url)
        .headers(headers)
        .header(reqwest::header::CONTENT_LENGTH, 0)
        .send()
        .await?;
    Ok(response)
}

pub async fn make_the_post(headers:reqwest::header::HeaderMap, url: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let client = reqwest::ClientBuilder::new()
    .danger_accept_invalid_certs(true) // Disable SSL verification
    .build()?;

    let response = client
        .post(url)
        .headers(headers)
        .header(reqwest::header::CONTENT_LENGTH, 0)
        .send()
        .await?;
    Ok(response)
}
pub struct Summary<'a> {
    pub config: &'a configuration::Config,
    pub summary: Box<serde_json::Value>,
}
pub async fn get_m42_object_from_summary(summary: Summary<'_>) -> serde_json::Value{
    // So many expectations :D
    let ciname = get_ciname_from_summary(&*summary.summary).await.expect("Oh shit");
    let objectid = get_id_from_summary(&*summary.summary).await.expect("Oh shit");
    let list_url = format!("data/objects/{}/{}?full=true", ciname, objectid);
    let url = format!("{}{}", &summary.config.api_endpoint, &list_url);
    let mut headers = None;
    if let Some(token) = &summary.config.raw_token {
        headers = Some(default_headers(token).await);
    }
    if let Some(h) = headers {
        let response = make_the_get(h, &url).await.expect("failed to get");
        return serde_json::from_str(&response.text().await.expect("parsing failure")).expect("dang");
    }
    else {
        panic!("fuuuuck");
    }
}

pub async fn get_ciname_from_summary<'a>(summary: &'a serde_json::Value) -> Result<&'a serde_json::Value, Box<dyn std::error::Error>> {
    let s = &summary["Sys-Entity"];
    if let &serde_json::Value::Null = s {
        return Err("no Sys-Entity field in summary".into());
    }
    Ok(s)
}

pub async fn get_id_from_summary<'a>(summary: &'a serde_json::Value) -> Result<&'a serde_json::Value, Box<dyn std::error::Error>> {
    let s = &summary["Sys-ObjectId"];
    if let &serde_json::Value::Null = s {
        return Err("no Sys-ObjectId field in summary".into());
    }
    Ok(s)
}

pub async fn default_headers(token: &str) -> reqwest::header::HeaderMap {
    let auth = format!("{}{}", "Bearer ", token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&auth).unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    return headers;
}

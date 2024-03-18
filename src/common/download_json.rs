use crate::parser::keyword::Root;
use reqwest::{self, header::USER_AGENT};
use serde_json::Value;

pub fn download(url: &str, parameters: Vec<(&str, &str)>) -> Value {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .query(&parameters)
        .header(
            USER_AGENT,
            format!("RSBBI (gh:Elsie19/rsbbi) v{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&response).unwrap()
}

pub fn post_download(url: &str, body: String, parameters: Vec<(&str, &str)>) -> Root {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .query(&parameters)
        .body(body)
        .header(
            USER_AGENT,
            format!("RSBBI (gh:Elsie19/rsbbi) v{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();

    serde_json::from_str(&response).unwrap()
}

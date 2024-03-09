use crate::parser::keyword::Root;
use reqwest;
use serde_json::Value;

pub fn download(url: &str, parameters: Vec<(&str, &str)>) -> Value {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .query(&parameters)
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
        .send()
        .unwrap()
        .text()
        .unwrap();

    let p: Root = serde_json::from_str(&response).unwrap();
    p
}

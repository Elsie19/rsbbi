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

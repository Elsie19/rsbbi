use reqwest;
use serde::{Deserialize, Serialize};

pub type Shape = Vec<ShapeContents>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeContents {
    pub section: String,
    pub he_title: String,
    pub title: String,
    pub length: i64,
    pub chapters: Vec<i64>,
    pub book: String,
    pub he_book: String,
}

pub fn shape_download(url: &str, parameters: Vec<(&str, &str)>) -> Shape {
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Search<'a> {
    pub query: String,
    #[serde(rename = "type")]
    pub query_type: &'a str,
    pub size: i32,
}

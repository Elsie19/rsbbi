use serde::{Deserialize, Serialize};

// The thing we send for `keyword`
#[derive(Debug, Deserialize, Serialize)]
pub struct Search<'a> {
    pub query: String,
    #[serde(rename = "type")]
    pub query_type: &'a str,
    pub size: i32,
}

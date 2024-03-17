use anyhow::anyhow;
use serde_json::Value;

pub fn convert_to_text(text: &Value) -> Result<Vec<&str>, anyhow::Error> {
    if text.is_string() {
        Ok([text.as_str().unwrap()].to_vec())
    } else if text.is_array() {
        let mut array_vec: Vec<&str> = vec![];
        for piece in text.as_array().unwrap() {
            // This is for chapter ranges
            if piece.is_array() {
                for part in piece.as_array().unwrap() {
                    array_vec.push(part.as_str().unwrap());
                }
            } else {
                array_vec.push(piece.as_str().unwrap());
            }
        }
        return Ok(array_vec);
    } else {
        return Err(anyhow!("Could convert 'text' to string or array: {}", text));
    }
}

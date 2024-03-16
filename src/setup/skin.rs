use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use termimad::MadSkin;

pub fn get_config(path: &PathBuf) -> MadSkin {
    let json = match fs::read_to_string(path) {
        Ok(yas) => yas,
        Err(_) => {
            let mut output = File::create(path).unwrap();
            let text = include_str!("../../extra/style.json").to_string();
            write!(output, "{}", text).unwrap();
            text
        }
    };

    serde_json::from_str(&json).unwrap()
}

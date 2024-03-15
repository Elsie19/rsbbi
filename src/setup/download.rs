use reqwest;
use std::env;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

pub fn setup_toc() {
    let mut resp = reqwest::blocking::get("https://www.sefaria.org/api/index/")
        .expect("Failed to download Table of Contents");
    let path: PathBuf = [
        env::var("HOME").unwrap().as_str(),
        ".local",
        "share",
        "rsbbi",
        "toc.json",
    ]
    .iter()
    .collect();

    if !Path::new(&path).exists() {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut out =
            File::create(path).expect("Could not write/open '~/.local/share/rsbbi/toc.json'");
        io::copy(&mut resp, &mut out).expect("Failed to copy toc");
    }
}

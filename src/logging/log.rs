use chrono::{Datelike, Timelike, Utc};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Log<'a> {
    file: &'a Path,
}

impl Log<'_> {
    pub fn new(path: &Path) -> Result<Log, std::io::Error> {
        let prefix = &path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        match OpenOptions::new().create(true).write(true).open(&path) {
            Ok(_) => Ok(Log { file: path }),
            Err(e) => Err(e),
        }
    }

    pub fn log(&self, text: Vec<serde_json::Value>) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.file)
            .unwrap();

        for line in text {
            writeln!(file, "{}", line).unwrap();
        }
    }
}

pub fn suggested_path() -> PathBuf {
    let now = Utc::now();

    let (year, month, day, hour, minute, seconds) = (
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    );
    let path: PathBuf = [
        env::var("XDG_STATE_HOME")
            .unwrap_or(env::var("HOME").unwrap())
            .as_str(),
        ".local",
        "state",
        "rsbbi",
        format!("{}-{}-{}", year, month, day).as_str(),
        format!("tetra-{}-{}-{}.log", hour, minute, seconds).as_str(),
    ]
    .iter()
    .collect();
    path
}

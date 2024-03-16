use chrono::{Datelike, Timelike, Utc};
use std::env;
#[cfg(feature = "tetragrammaton-logging")]
use std::fs::OpenOptions;
#[cfg(feature = "tetragrammaton-logging")]
use std::io::prelude::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Log<'a> {
    file: &'a Path,
}

impl Log<'_> {
    #[cfg(feature = "tetragrammaton-logging")]
    pub fn new(path: &Path) -> Result<Log, std::io::Error> {
        let prefix = &path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(Log { file: path }),
            Err(e) => Err(e),
        }
    }

    #[cfg(not(feature = "tetragrammaton-logging"))]
    pub fn new(path: &Path) -> Result<Log, std::io::Error> {
        Ok(Log { file: path })
    }

    #[allow(unused_variables)]
    pub fn log(&self, text: Vec<&str>) {
        #[cfg(feature = "tetragrammaton-logging")]
        {
            let mut file = OpenOptions::new().append(true).open(self.file).unwrap();

            for line in text {
                writeln!(file, "{line}").unwrap();
            }
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
        format!("{year}-{month}-{day}").as_str(),
        format!("tetra-{hour}-{minute}-{seconds}.log").as_str(),
    ]
    .iter()
    .collect();
    path
}

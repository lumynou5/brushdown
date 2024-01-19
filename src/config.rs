use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ConfigRaw {
    pub src: Option<String>,
    pub dest: Option<String>,
}

#[derive(Serialize)]
pub struct Config {
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Config {
    pub fn from_raw(raw: ConfigRaw) -> Self {
        Config {
            src: PathBuf::from(&raw.src.unwrap_or("src".to_string())),
            dest: PathBuf::from(&raw.dest.unwrap_or("dest".to_string())),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            src: PathBuf::from("src"),
            dest: PathBuf::from("dest"),
        }
    }
}

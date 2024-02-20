use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub src: PathBuf,
    pub dest: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            src: PathBuf::from("src"),
            dest: PathBuf::from("dest"),
        }
    }
}

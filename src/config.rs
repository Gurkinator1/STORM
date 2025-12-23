use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub devices: Vec<PathBuf>,
}

impl Config {
    pub fn contains_drive(&self, path: &PathBuf) -> bool {
        self.devices.contains(path)
    }
}

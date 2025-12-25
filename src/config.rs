use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub makemkvcon_path: Option<PathBuf>,
    pub ffmpeg_path: Option<PathBuf>,
    pub devices: Vec<PathBuf>,
    pub keys: Keys,
}

#[derive(Deserialize)]
pub struct Keys {
    pub tmdb: String,
}

impl Config {
    pub fn contains_drive(&self, path: &PathBuf) -> bool {
        self.devices.contains(path)
    }
}

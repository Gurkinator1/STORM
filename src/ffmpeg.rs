use std::path::PathBuf;

use anyhow::anyhow;
use log::info;
use which::which;

pub struct Ffmpeg {
    pub path: PathBuf,
}

impl Ffmpeg {
    pub fn new(path: &Option<PathBuf>) -> anyhow::Result<Self> {
        let path = path.clone().or_else(|| which("ffmpeg").ok());
        if let Some(path) = path {
            info!("using ffmpeg at {}", path.to_string_lossy());
            return Ok(Ffmpeg { path: path });
        } else {
            return Err(anyhow!("ffmpeg not found!"));
        }
    }
}

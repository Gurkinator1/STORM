use std::path::PathBuf;

use anyhow::{Ok, anyhow};
use log::{error, info};
use tokio::process::Command;
use which::which;

pub struct MakeMKV {
    pub path: Option<PathBuf>,
}

impl MakeMKV {
    pub async fn new(path: &Option<PathBuf>) -> anyhow::Result<Self> {
        let path = path.clone().or_else(|| which("makemkvcon").ok());

        if let Some(path) = path {
            info!("using makemkvcon at {}", path.to_string_lossy());
            return Ok(MakeMKV { path: Some(path) });
        } else {
            let out = Command::new("flatpak")
                .arg("info")
                .arg("com.makemkv.MakeMKV")
                .output()
                .await;

            if let Result::Ok(out) = out {
                if out.status.success() {
                    info!("using flatpak makemkvcon");
                    return Ok(MakeMKV { path: None });
                }
            }

            error!("no makemkvcon found!");
            return Err(anyhow!("flatpak failed"));
        }
    }
}

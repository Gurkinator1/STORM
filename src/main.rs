use anyhow::anyhow;
use clap::{Parser, Subcommand};
use directories::BaseDirs;
use eject::{device::Device, discovery::cd_drives};
use rocket::{Rocket, get, routes};
use std::{path::PathBuf, process::exit};
use tokio::fs;

use crate::{config::Config, ffmpeg::Ffmpeg, makemkv::MakeMKV, worker::Worker};

mod config;
mod ffmpeg;
mod makemkv;
mod udev;
mod worker;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    Info,
    Serve,
    Default,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let config_path: PathBuf = args.config.unwrap_or_else(|| {
        let dirs = BaseDirs::new().expect("failed to get home directory");
        dirs.config_dir().join("storm.toml")
    });

    match &args.command {
        Commands::Info => {
            //config
            println!("config path: {}", config_path.to_string_lossy());
            let cfg = get_config(&config_path).await;

            //makemkv
            if let Ok(mkv) = MakeMKV::new(&cfg.makemkvcon_path).await {
                println!(
                    "found makemkv: {}",
                    mkv.path
                        .map(|v| v.to_string_lossy().to_string())
                        .unwrap_or("flatpak".to_string())
                );
            } else {
                println!("makemkv not found!");
            }

            //ffmpeg
            if let Ok(fg) = Ffmpeg::new(&cfg.ffmpeg_path) {
                println!("found ffmpeg: {}", fg.path.to_string_lossy());
            } else {
                println!("ffmpeg not found!");
            }

            //drives
            let drives: Vec<PathBuf> = cd_drives().collect();
            println!("detected drives: {}", drives.len());
            for drive in drives {
                println!(
                    "[{}] {}",
                    if cfg.contains_drive(&drive) { "*" } else { " " },
                    drive.to_string_lossy()
                );
            }
        }
        Commands::Default => {
            fs::write(&config_path, include_bytes!("./default.toml"))
                .await
                .expect("failed to write default config");
            println!("successfully written default config to:");
            println!("{}", config_path.to_string_lossy());
        }

        Commands::Serve => {
            let cfg = get_config(&config_path).await;

            //init udev monitor
            let mon = crate::udev::monitor().unwrap();

            //init workers
            let mut workers: Vec<Worker> = Vec::new();
            for dev_path in cfg.devices {
                if let Ok(dev) = Device::open(&dev_path) {
                    workers.push(Worker::new(mon.resubscribe(), dev));
                } else {
                    return Err(anyhow!("failed to open: {}", dev_path.to_string_lossy()));
                }
            }

            //startup webserver
            let _ = build_rocket()
                .ignite()
                .await
                .expect("Rocket failed to ignite")
                .launch()
                .await;
        }
    }
    Ok(())
}

async fn get_config(config_path: &PathBuf) -> Config {
    match fs::read_to_string(config_path).await {
        Ok(content) => match toml::from_str(&content) {
            Ok(config) => return config,
            Err(e) => {
                println!("failed to parse config!");
                println!("{}", e.message());
                exit(-1);
            }
        },
        Err(e) => {
            println!("failed to open config file: {}", e);
            exit(-1);
        }
    }
}

fn build_rocket() -> rocket::Rocket<rocket::Build> {
    Rocket::build().mount("/", routes![web_root])
}

#[get("/")]
fn web_root() -> String {
    "hello world!".to_string()
}

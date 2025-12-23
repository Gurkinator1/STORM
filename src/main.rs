use clap::{Parser, Subcommand};
use directories::BaseDirs;
use std::{path::PathBuf, thread};
use tokio::fs;

use crate::config::Config;

mod config;
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config_path: PathBuf = args.config.unwrap_or_else(|| {
        let dirs = BaseDirs::new().expect("failed to get home directory");
        dirs.config_dir().join("storm.toml")
    });

    match &args.command {
        Commands::Info => {
            println!("config path: {}", config_path.to_string_lossy());
        }
        Commands::Serve => {
            let config: Config = toml::from_str(
                &fs::read_to_string(config_path)
                    .await
                    .expect("failed to read config file"),
            )
            .expect("config invalid");

            //init udev monitor
            let mon = crate::udev::monitor().unwrap();
        }
    }
    Ok(())
}

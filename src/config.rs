use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    port: u16,
    devices: Vec<String>,
}

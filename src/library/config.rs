use serde::Deserialize;
use std::fs;
#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u16,
}
#[derive(Deserialize, Debug)]
pub struct Config{
    pub server: Server,
}
pub fn config() -> Option<Config> {
    match fs::read_to_string("config.toml") {
        Ok(config) => {
            if let Ok(config) = toml::from_str::<Config>(&config) {
                Some(config)
            }
            else { 
                None
            }
        },
        Err(_) => None,
    }
}
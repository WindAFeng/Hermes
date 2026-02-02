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
pub fn config() -> Config {
    fn define_config() -> Config {
        Config{
            server: Server {
                host: String::from("127.0.0.1"),
                port: 6657,
            }
        }
    }
    match fs::read_to_string("config.toml") {
        Ok(config) => {
            if let Ok(config) = toml::from_str::<Config>(&config) {
                config
            }
            else {
                define_config()
            }
        },
        Err(_) => define_config(),
    }
}
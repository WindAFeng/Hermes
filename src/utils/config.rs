use crate::utils::log::{error, info, warn};
use crate::models::config::Config;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;
use crate::models::default_config::DefaultConfig;


const CONFIG_FILE: &str = "Config.toml";
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub fn init_config() {
    let mut config = Config::default();

    match fs::read_to_string(CONFIG_FILE) {
        Ok(contents) => {
            match toml::from_str::<Config>(&contents) {
                Ok(parsed) => {
                    info("loaded config");
                    config = parsed;
                }
                Err(error) => warn(format!("error while parsing config: {}", error).as_str())
            }
        }
        Err(_) => {
            let toml_str = match toml::to_string_pretty(&DefaultConfig::default()) {
                Ok(toml_str) => toml_str,
                Err(e) => {
                    error(format!("{}", e.to_string()));
                    return ()
                },
            };
            let write = fs::write(CONFIG_FILE, toml_str);
            match write {
                Ok(_) => (),
                Err(error) => {
                    warn(error.to_string());
                }
            }
        }
    }
    let _ = CONFIG.set(Arc::new(config));
}

pub fn get_config() -> Arc<Config> {
    CONFIG.get().cloned().unwrap_or_else(|| {
        Arc::new(Config::default())
    })
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_config() {
        init_config();
        let config = get_config();
        println!("{:?}", config);
    }
}
use crate::models::config::Config;
use crate::models::hermes_model::hermes_default_config::HermesDefaultConfig;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;
use tracing::log::{error, info, warn};

const CONFIG_FILE: &str = "Config.toml";
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

pub fn init_config() {
    let mut config = Config::default();
    if let Ok(contents) = fs::read_to_string(CONFIG_FILE) {
        match toml::from_str::<Config>(&contents) {
            Ok(parsed) => {
                info!("loaded config");
                config = parsed;
            }
            Err(error) => warn!("error while parsing config: {}", error),
        }
    } else {
        let toml_str = match toml::to_string_pretty(&HermesDefaultConfig::default()) {
            Ok(toml_str) => toml_str,
            Err(e) => {
                error!("{}", e.to_string());
                return ();
            }
        };
        fs::write(CONFIG_FILE, toml_str).unwrap_or_else(|e| warn!("{}", e.to_string()));
    }
    let _ = CONFIG.set(Arc::new(config));
}

pub fn get_config() -> Arc<Config> {
    CONFIG
        .get()
        .cloned()
        .unwrap_or_else(|| Arc::new(Config::default()))
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

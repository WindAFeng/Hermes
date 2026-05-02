use crate::utils::log::{error, info, warn};
use crate::config::Config;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;
use crate::default_config::DefaultConfig;

// --- 全局变量 ---
const CONFIG_FILE: &str = "Config.toml";
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

// --- 2. 通用初始化函数 (以后再也不用改了!) ---
pub fn init_config() {
    let mut config = Config::default();

    // 2. 尝试读取文件覆盖默认值
    if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
        if let Ok(parsed) = toml::from_str::<Config>(&content) {
            info("loaded config");
            config = parsed; // 文件配置优先级高
        } else if let Err(error) = toml::from_str::<Config>(&content) {
            warn(format!("error while parsing config: {}", error).as_str());
        }
    } else {
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

    // 3. 设置全局
    let _ = CONFIG.set(Arc::new(config));
}

// --- 获取函数 (不变) ---
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
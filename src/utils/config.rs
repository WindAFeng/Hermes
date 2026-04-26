use crate::utils::log::{info, warn, error};
use crate::config::Config;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;


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
        } else {
            warn("cannot parse config file");
        }
    } else {
        warn("not found config file");
    }

    // 3. 设置全局
    let _ = CONFIG.set(Arc::new(config));
}

// --- 获取函数 (不变) ---
pub fn get_config() -> Arc<Config> {
    CONFIG.get().cloned().unwrap_or_else(|| {
        error("配置未初始化");
        Config::default().into()
    })
}
use crate::log::{info, warn, error};
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;

// --- 1. 定义结构体 (只需这里改) ---
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Server {
    #[serde(default = "default_socket_host")]
    pub socket_host: String,
    #[serde(default = "default_socket_port")]
    pub socket_port: u16,
    #[serde(default = "default_websocket_host")]
    pub websocket_host: String,
    #[serde(default = "default_websocket_port")]
    pub websocket_port: u16,
}

// 辅助函数：定义默认值
fn default_socket_host() -> String { "127.0.0.1".to_string() }
fn default_socket_port() -> u16 { 6657 }
fn default_websocket_host() -> String { "127.0.0.1".to_string() }
fn default_websocket_port() -> u16 { 6658 }

// 创建结构体
#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
}

// --- 全局变量 ---
const CONFIG_FILE: &str = "config.toml";
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

// --- 2. 通用初始化函数 (以后再也不用改了!) ---
pub fn init_config() {
    let mut config = Config::default();

    // 2. 尝试读取文件覆盖默认值
    if let Ok(content) = fs::read_to_string(CONFIG_FILE) {
        if let Ok(parsed) = toml::from_str::<Config>(&content) {
            info("成功加载配置文件");
            config = parsed; // 文件配置优先级高
        } else {
            warn("无法读取配置文件，已启用默认配置");
        }
    } else {
        warn("配置文件不存在，已启用默认配置。");
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
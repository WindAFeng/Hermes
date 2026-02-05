use crate::log::{info, warn, error}; // 引入你刚才写的日志函数
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use std::sync::OnceLock;

// --- 结构体定义 ---
#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: Server,
}

// --- 全局静态变量 ---
const CONFIG_FILE: &str = "config.toml";
static CONFIG: OnceLock<Arc<Config>> = OnceLock::new();

// --- 初始化函数 ---
pub fn init_config() {
    let default_config = Config {
        server: Server {
            host: String::from("127.0.0.1"),
            port: 6657,
        },
    };

    // 尝试读取文件
    match fs::read_to_string(CONFIG_FILE) {
        Ok(content) => {
            // 尝试解析 TOML
            match toml::from_str::<Config>(&content) {
                Ok(parsed) => {
                    info(format!("配置: 成功从文件 '{}' 加载", CONFIG_FILE));
                    // 安全设置全局变量
                    let _ = CONFIG.set(Arc::new(parsed));
                }
                Err(e) => {
                    warn(format!("配置解析错误: {}. 使用默认配置.", e));
                    let _ = CONFIG.set(Arc::new(default_config));
                }
            }
        }
        Err(e) => {
            warn(format!("无法读取配置文件 '{}': {}. 使用默认配置.", CONFIG_FILE, e));
            let _ = CONFIG.set(Arc::new(default_config));
        }
    }
}

// --- 获取函数 ---
pub fn get_config() -> Arc<Config> {
    if let Some(config) = CONFIG.get() {
        return config.clone();
    }
    error("尝试获取配置时，配置尚未初始化！请检查是否在 main 函数开头调用了 init_config()");
    Arc::new(Config {
        server: Server {
            host: String::from("127.0.0.1"),
            port: 6657,
        },
    })
}
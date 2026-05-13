use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub web: WebConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ServerConfig {
    #[serde(default)]
    pub socket: SocketConfig,
    #[serde(default)]
    pub websocket: WebsocketConfig,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub mysql: HashMap<String, MysqlConfig>,
    #[serde(default)]
    pub postgres: HashMap<String, PostgresConfig>,
    #[serde(default)]
    pub redis: HashMap<String, RedisConfig>,
    #[serde(default)]
    pub mongodb: HashMap<String, MongoConfig>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct MysqlConfig{
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub priority: u8,
}
#[derive(Deserialize, Debug, Clone)]
pub struct PostgresConfig{
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub priority: u8,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RedisConfig{
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub priority: u8,
}
#[derive(Deserialize, Debug, Clone)]
pub struct MongoConfig{
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocketConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_socket_port")]
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebsocketConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_websocket_port")]
    pub port: u16,
}
#[derive(Debug, Deserialize, Clone)]
pub struct WebConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_web_port")]
    pub port: u16,
}
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_socket_port() -> u16 {
    6657
}
fn default_websocket_port() -> u16 {
    6658
}
fn default_web_port() -> u16 {
    8080
}
impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_socket_port(),
        }
    }
}

impl Default for WebsocketConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_websocket_port(),
        }
    }
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_web_port(),
        }
    }
}

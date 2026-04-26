use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub web: WebConfig,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ServerConfig {
    pub socket: SocketConfig,
    pub websocket: WebsocketConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SocketConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_socket_port")]
    pub port: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WebsocketConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_websocket_port")]
    pub port: String,
}
#[derive(Debug, Deserialize, Clone, Default)]
pub struct WebConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_web_port")]
    pub port: u16,
}
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_socket_port() -> String {
    "6657".to_string()
}
fn default_websocket_port() -> String {
    "6658".to_string()
}
fn default_web_port() -> u16 {
    8080
}

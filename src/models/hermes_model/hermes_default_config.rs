use serde::Serialize;

#[derive(Serialize)]
pub struct HermesDefaultConfig{
    pub server: HermesDefaultServerConfig,
    pub web: HermesDefaultWebConfig,
}
#[derive(Serialize)]
pub struct HermesDefaultServerConfig {
    pub socket: HermesDefaultSocketConfig,
    pub websocket: HermesDefaultWebsocketConfig,
}
#[derive(Serialize)]
pub struct HermesDefaultSocketConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize)]
pub struct HermesDefaultWebsocketConfig {
    pub host: String,
    pub port: u16,
}
#[derive(Serialize)]
pub struct HermesDefaultWebConfig {
    pub host: String,
    pub port: u16,
}
impl HermesDefaultConfig{
    pub fn default() -> Self{
        Self {
            server: HermesDefaultServerConfig {
                socket: HermesDefaultSocketConfig {
                    host: "127.0.0.1".to_string(),
                    port: 6657,
                },
                websocket: HermesDefaultWebsocketConfig {
                    host: "127.0.0.1".to_string(),
                    port: 6658,
                },
            },
            web: HermesDefaultWebConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            }
        }
    }
}
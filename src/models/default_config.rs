use serde::Serialize;

#[derive(Serialize)]
pub struct DefaultConfig{
    pub(crate) server: DefaultServerConfig,
    pub(crate) web: DefaultWebConfig,
}
#[derive(Serialize)]
pub struct DefaultServerConfig {
    pub socket: DefaultSocketConfig,
    pub websocket: DefaultWebsocketConfig,
}
#[derive(Serialize)]
pub struct DefaultSocketConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Serialize)]
pub struct DefaultWebsocketConfig {
    pub host: String,
    pub port: u16,
}
#[derive(Serialize)]
pub struct DefaultWebConfig {
    pub host: String,
    pub port: u16,
}
impl DefaultConfig{
    pub fn default() -> Self{
        Self {
            server: DefaultServerConfig {
                socket: DefaultSocketConfig {
                    host: "127.0.0.1".to_string(),
                    port: 6657,
                },
                websocket: DefaultWebsocketConfig {
                    host: "127.0.0.1".to_string(),
                    port: 6658,
                },
            },
            web: DefaultWebConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            }
        }
    }
}
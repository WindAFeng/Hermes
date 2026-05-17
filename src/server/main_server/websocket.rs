use crate::models::hermes_model::hermes_error::HermesError;
use crate::server::main_server::handel_websocket::websocket_handle;
use crate::server::main_server::tcp::Tcp;
pub struct WebsocketServer {
    tcp_server: Tcp,
}
impl WebsocketServer {
    pub async fn new(host: &str, port: &str) -> Result<Self, HermesError> {
        Tcp::new(host, port, "WebSocket")
            .await
            .map(|tcp_server| Self { tcp_server })
    }
    pub async fn start(&mut self) {
        self.tcp_server.run(websocket_handle).await;
    }
}

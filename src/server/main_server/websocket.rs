use crate::errors::HermesError;
use crate::server::main_server::tcp::Tcp;
use crate::server::main_server::handel_websocket::websocket_handel;
pub struct WebsocketServer {
    tcp_server: Tcp
}
impl WebsocketServer {
    pub async fn new(host: &str, port: &str) -> Result<WebsocketServer, HermesError>{
        match Tcp::new(host, port, "WebSocket").await {
            Ok(tcp_server) => {
                Ok(WebsocketServer{
                    tcp_server
                })
            },
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn start(&mut self){
        self.tcp_server.run(websocket_handel).await;
    }
}
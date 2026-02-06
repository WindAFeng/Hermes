use crate::library::config::get_config;
use crate::server::{socket::SocketServer, websocket::WebsocketServer};


pub struct Server {
    websocket_status: bool,
    websocket_addr: String,
    socket_addr: String,
}
impl Server {
    pub fn new() -> Server {
        let server_config = &get_config().server;
        Server {
            websocket_status: false,
            websocket_addr: format!("{}:{}", server_config.websocket_host, server_config.websocket_port),
            socket_addr: format!("{}:{}", server_config.socket_host, server_config.socket_port),
        }
    }
    pub fn enable_websocket(mut self) -> Self {
        self.websocket_status = true;
        self
    }
    pub async fn run(self) {
        let mut socket_server = SocketServer::new(self.socket_addr);

        if self.websocket_status {
            let mut websocket_server = WebsocketServer::new(self.websocket_addr);
            tokio::join!(
                websocket_server.run(),
                socket_server.run()
            );
        } else {
            socket_server.run().await;
        }
    }
}

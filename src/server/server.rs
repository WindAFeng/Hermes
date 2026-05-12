use tokio::signal;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::server::main_server::{websocket::WebsocketServer, socket::SocketServer};
use crate::utils::{config::get_config, log};
use crate::server::api::web::WebApp;

pub struct MainServer {
    socket_host: String,
    socket_port: String,
    websocket_host: String,
    websocket_port: String,
}

impl MainServer {
    pub fn new() -> Self {
        let server_config = &get_config().server;
        Self {
            socket_host: server_config.socket.host.clone(),
            socket_port: server_config.socket.port.clone().to_string(),
            websocket_host: server_config.websocket.host.clone(),
            websocket_port: server_config.websocket.port.clone().to_string(),
        }
    }

    pub async fn run(&self) -> Result<(), HermesError> {
        let socket_fut = {
            let mut socket_server = SocketServer::new(&self.socket_host, &self.socket_port).await?;
            tokio::spawn(async move {
                socket_server.start().await
            })
        };
        let ws_fut = {
            let mut ws_server = WebsocketServer::new(&self.websocket_host, &self.websocket_port).await?;
            tokio::spawn(async move {
                ws_server.start().await
            })
        };
        let http_fut = {
            let web = WebApp::new();
            tokio::spawn(async move {
                web.start().await
            })
        };
        tokio::select! {
            res = socket_fut => {
                log::error(format!("Socket server exited: {:?}", res));
            },
            res = ws_fut => {
                log::error(format!("WebSocket server exited: {:?}", res));
            },
            res = http_fut => {
                log::error(format!("HTTP (actix-web) server exited: {:?}", res));
            },
            _ = signal::ctrl_c() => {
                log::info("Received Ctrl+C, shutting down...");
                return Ok(());
            }
        }
        Ok(())
    }
}
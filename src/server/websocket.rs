use crate::server::tcp::TcpServer;
use tokio_tungstenite::{accept_async};
use futures_util::StreamExt;
use tokio::io;
use crate::library::config::Server;
use crate::library::log;
use crate::model::Request;

pub struct WebsocketServer {
    tcp_server: TcpServer
}

impl WebsocketServer {
    pub fn new(addr: String) -> WebsocketServer {
        WebsocketServer {
            tcp_server: TcpServer::new(addr)
        }
    }

    pub async fn run(&mut self) {
        log::info("Websocket Server started");
        self.tcp_server.run(handle_ws_client).await;
    }
}
async fn handle_ws_client(stream: tokio::net::TcpStream) -> std::io::Result<()> {
    let ws_stream = match accept_async(stream).await {
        Ok(stream) => {
            println!("WebSocket 握手成功");
            stream
        }
        Err(e) => {
            eprintln!("WebSocket 握手失败: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();
    loop {
        if let Some(message_result) = receiver.next().await {
            let message = match message_result {
                Ok(message) => {
                    if message.is_close() {
                        break;
                    }
                    message
                },
                Err(e) => {
                    log::error(format!("MessageError: {}", e));
                    break;
                }
            };
            if message.is_text() {
                let text = message.to_text().map_err( |_| {
                    io::Error::new(io::ErrorKind::InvalidData, "消息不是有效的文本")
                })?;
                match serde_json::from_str::<Request>(text) {
                    Ok(request) => {
                        // 成功解析请求
                    }
                    Err(e) => {
                        log::error(format!("JSON解析失败: {}", e))
                    }
                }
            }else if message.is_close() {
                // 如果收到关闭帧，退出
                break;
            }

        }
    }
    Ok(())
}
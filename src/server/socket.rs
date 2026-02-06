use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::server::tcp::TcpServer;
use tokio::net::{TcpStream};
use crate::library::log;
use crate::model::{Request, Response};

pub struct SocketServer {
    tcp_server: TcpServer
}
impl SocketServer {
    pub fn new(addr: String) -> SocketServer {
        SocketServer {
            tcp_server: TcpServer::new(addr)
        }
    }
    pub async fn run(&mut self) {
        log::info("Socket Server started");
        self.tcp_server.run(handle_socket_client).await;
    }
}
async fn handle_socket_client(mut socket: TcpStream) -> std::io::Result<()>{
    let mut buf = [0; 4096];
    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => break, // 连接关闭
            Ok(n) => n,
            Err(e) => {
                log::warn(format!("读取数据错误: {}", e));
                break;
            }
        };
        
        let req_result: Result<Request, _> = serde_json::from_slice(&buf[..n]);
        let req = match req_result {
            Ok(r) => r,
            Err(e) => {
                log::error(format!("JSON解析失败: {}", e));
                continue;
            }
        };

        let resp = Response {
            code: 0,
            message: "success".to_string(),
            data: None,
        };

        let resp_bytes = serde_json::to_vec(&resp)?;
        let _ = socket.write_all(&resp_bytes).await;
    }
    Ok(())
}
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::model::{Request, Response};
use crate::library::{logging::log};
pub(crate) struct TcpServer {
    host: String,
    port: u16,
}
impl TcpServer {
    pub fn new(host: String, port: u16) -> Self {
        // 判断是否传入port参数并存在默认port=6657
        Self {
            host,
            port,
        }
    }
    pub fn on_websocket(&self) {
        // 创建对websocket的支持
        todo!()
    }
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 构建标准的 Socket 地址
        let addr = format!("{}:{}", self.host, self.port);
        // 绑定 Socket 主机地址
        let listener = TcpListener::bind(&addr).await?;
        // 循环接收请求
        self.accept_loop(listener).await
    }
    async fn accept_loop(
        &self,
        listener: TcpListener
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    log::debug(format!("新连接来自: {}", peer_addr).as_str());
                    tokio::spawn(handle_client(socket));
                }
                Err(e) => {
                    log::error(format!("连接出错:{}", e).as_str());
                }
            }
        }
    }
}
async fn handle_client(mut socket: TcpStream) {
    // 创建4kb缓冲
    let mut buf = [0u8; 4096];

    if let Err(e) = async {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "空数据",
            ));
        }

        let req: Request = serde_json::from_slice(&buf[..n])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        

        let resp = Response {
            code: 0,
            message: "success".to_string(),
            data: None,
        };

        let resp_bytes = serde_json::to_vec(&resp)?;
        socket.write_all(&resp_bytes).await?;
        Ok::<(), std::io::Error>(())
    }
    .await
    {
        eprintln!("处理客户端时出错: {}", e);
    }
}

use tokio::net::{TcpListener, };
use crate::library::{logging::log};
use crate::tcp::handle_client::handle_client;
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
    #[warn(dead_code)]
    pub fn on_websocket(&self) {
        // 创建对websocket的支持
        todo!()
    }
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 构建标准的 Socket 地址
        let addr = format!("{}:{}", self.host, self.port);
        // 绑定 Socket 主机地址
        let listener = TcpListener::bind(&addr).await?;
        log::info("Hermes系统启动成功");
        log::info(format!("系统运行在: {}", addr).as_str());
        // 循环接收请求
        self.accept_loop(listener).await
    }
    async fn accept_loop(&self, listener: TcpListener)
        -> Result<(), Box<dyn std::error::Error>> {
        // 开启循环接收连接
        loop {
            // 匹配连接
            match listener.accept().await {
                // 连接成功
                Ok((socket, peer_addr)) => {
                    log::debug(format!("新连接来自: {}", peer_addr).as_str());
                    tokio::spawn(handle_client(socket));
                }
                // 连接失败
                Err(e) => {
                    log::error(format!("连接出错:{}", e).as_str());
                }
            }
        }
    }
}
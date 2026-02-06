use tokio::net::{TcpListener, TcpStream};
use crate::log;
pub(super) struct TcpServer {
    addr: String,
}
impl TcpServer{
    pub fn new(addr: String)->Self{
        Self {
            addr
        }
    }
    pub async fn run<F, Fut>(&self, handle: F)
    where
        F: Fn(TcpStream) -> Fut,  // F 是一个函数，输入 TcpStream，返回 Fut
        Fut: Future<Output = std::io::Result<()>> + Send + 'static {
        let listener = match TcpListener::bind(&self.addr).await {
            Ok(l) => l,
            Err(e) => {
                log::error(format!("绑定地址失败: {}: {}", self.addr, e));
                return; // 或者 panic!
            }
        };
        self.accept_loop(listener, handle).await;
    }
    async fn accept_loop<F, Fut>(&self, listener: TcpListener, handle: F)
    where
        F: Fn(TcpStream) -> Fut,
        Fut: Future<Output = std::io::Result<()>> + Send + 'static{
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    log::debug(format!("新连接客户端:{}", addr));
                    tokio::spawn(handle(socket));
                }
                Err(e) => {
                    log::error(format!("连接出错: {}", e));
                }
            }
        }
    }
}

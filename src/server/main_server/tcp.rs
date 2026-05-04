use crate::errors::HermesError;
use crate::utils::{local_host, log};
use tokio::net::{TcpListener, TcpStream};
async fn create_tcp_listener(
    host: &str,
    port: &str,
    addr: String,
    socket_type: &str,
) -> Result<TcpListener, HermesError> {
    match TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("{} Listening on:", socket_type);
            if host == "0.0.0.0" {
                let local_host = local_host::get_local_host()?;
                println!(" - https://{}:{}", local_host, port);
                println!(" - https://127.0.0.1:{}", port);
            } else {
                println!(" - https://127.0.0.1:{}", port);
            }
            Ok(listener)
        }
        Err(e) => Err(HermesError::from(e)),
    }
}
pub struct Tcp {
    tcp_listener: TcpListener,
}
impl Tcp {
    pub async fn new(host: &str, port: &str, socket_type: &str) -> Result<Self, HermesError> {
        let addr = format!("{}:{}", host, port);
        match create_tcp_listener(&host, &port, addr, socket_type).await {
            Ok(listener) => {
                Ok(Self { tcp_listener: listener })
            }
            Err(e) => Err(HermesError::from(e)),
        }
    }
    pub async fn run<F, FUT>(&self, handel: F)
    where
        F: Fn(TcpStream) -> FUT,
        FUT: Future<Output = Result<(), HermesError>> + Send + 'static,
    {
        loop {
            match self.tcp_listener.accept().await {
                Ok((socket, addr)) => {
                    log::debug(format!("Connection from {:?}", addr));
                    tokio::spawn(handel(socket));
                }
                Err(e) => {
                    log::error(format!("Connect Error: {:?}", e));
                }
            }
        }
    }
}

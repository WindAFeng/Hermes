use tokio::net::{TcpListener, TcpStream};
use crate::errors::HermesError;
use crate::utils::{log, local_host};
pub struct Tcp {
    tcp_listener: TcpListener,
}
impl Tcp {
    pub async fn new(host: &str, port: &str) -> Result<Tcp, HermesError> {
        let addr = format!("{}:{}", host, port);
        match TcpListener::bind(addr).await {
            Ok(listener) => {
                println!("Listening on:");
                if host == "0.0.0.0" {
                    let local_host = local_host::get_local_host()?;
                    println!("https://{}:{}", local_host, port);
                    println!("https://127.0.0.1:{}", port);
                }
                else {
                    println!("https://127.0.0.1:{}", port);
                }
                Ok(Tcp { tcp_listener: listener })
            }
            Err(e) => Err(HermesError::from(e)),
        }
    }
    pub async fn run<F, FUT>(&self, handel: F)
    where F: Fn(TcpStream) -> FUT,
          FUT: Future<Output = Result<(), HermesError>> + Send + 'static{
        loop {
            match self.tcp_listener.accept().await {
                Ok((socket, addr)) => {
                    log::debug(format!("Connection from {:?}", addr));
                    tokio::spawn(
                        handel(socket)
                    );
                },
                Err(e) => {
                    log::error(format!("Connect Error: {:?}", e));
                }
            }
        }
    }
}
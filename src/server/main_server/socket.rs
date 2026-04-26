use crate::errors::HermesError;
use crate::server::main_server::tcp::Tcp;
use crate::server::main_server::socket_handel::socket_handel;
pub struct SocketServer{
    tcp_server: Tcp
}
impl SocketServer{
    pub async fn new(host: &str, port: &str) -> Result<SocketServer, HermesError>{
        match Tcp::new(host, port).await {
            Ok(tcp_server) => {
                Ok(SocketServer {
                    tcp_server
                })
            }
            Err(e) => {
                Err(e)
            }
        }
    }
    pub async fn start(&mut self){
        self.tcp_server.run(socket_handel).await;
    }
}
use crate::models::hermes_model::hermes_error::HermesError;
use crate::server::main_server::handel_socket::socket_handel;
use crate::server::main_server::tcp::Tcp;
pub struct SocketServer {
    tcp_server: Tcp,
}
impl SocketServer {
    pub async fn new(host: &str, port: &str) -> Result<Self, HermesError> {
        Tcp::new(host, port, "Socket")
            .await
            .map(|tcp_server| Self { tcp_server })
    }
    pub async fn start(&mut self) {
        self.tcp_server.run(socket_handel).await;
    }
}

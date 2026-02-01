mod tcp;
mod command_resolution;
mod model;
mod library;
use tcp::tcp_server::TcpServer;
#[tokio::main]
async fn main() {
    let tcp_server: TcpServer = TcpServer::new(String::from("127.0.0.1"), 6657);
    tcp_server.run().await.expect("TODO: panic message");
}

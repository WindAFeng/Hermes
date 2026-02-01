mod tcp;
mod command_resolution;
mod model;
mod library;
use tcp::tcp_server::TcpServer;
use library::{logging::log, config::{config, Config, Server}};
#[tokio::main]
async fn main() {
    let cfg = config().unwrap_or(Config {
        server: Server {
            host: "127.0.0.1".to_string(),
            port: 8080,
        }
    });
    let host: String = cfg.server.host;
    let port: u16 = cfg.server.port;
    let tcp_server: TcpServer = TcpServer::new(host, port);
    match tcp_server.run().await {
        Ok(_) => (),
        Err(e) => {
            log::error(format!("系统出错: {}", e).as_str());
        },
    }
}

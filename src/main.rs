mod tcp;
mod command_resolution;
mod model;
mod library;
use tcp::tcp_server::TcpServer;
use library::{logging::log, config::config};
#[tokio::main]
async fn main() {
    let cfg = config();
    let tcp_server: TcpServer = TcpServer::new(cfg.server.host, cfg.server.port);
    match tcp_server.run().await {
        Ok(_) => (),
        Err(e) => {
            log::error(format!("系统出错: {}", e).as_str());
        },
    }
}

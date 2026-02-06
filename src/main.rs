mod server;
mod command_resolution;
mod model;
mod library;
mod sql_builder;
mod database;
mod actuator;
mod db_pool_manager;

use library::{log, config::init_config};
use crate::server::server::Server;

#[tokio::main]
async fn main() {
    log::debug("正在启动服务");
    // 初始化配置文件
    init_config();
    // 启动服务
    let server = Server::new()
        .enable_websocket();
    server.run().await;
}

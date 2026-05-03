use crate::errors::HermesError;
use crate::server::server::MainServer;
use crate::utils::{config::init_config, log};
mod redis_lib;
pub mod models;
pub mod errors;
pub mod utils;
pub mod server;
pub mod command_handle;

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    log::debug("Welcome to Hermes system");
    log::debug("Hermes Server started");
    init_config();
    let main_server = MainServer::new();
    main_server.run().await
}
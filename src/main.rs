use models::hermes_model::hermes_error::HermesError;
use crate::server::server::MainServer;
use crate::utils::{config::init_config, log};
pub mod models;
pub mod utils;
pub mod server;
pub mod command_executor;
pub mod database_lib;

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    log::debug("Welcome to Hermes system");
    log::debug("Hermes Server started");
    init_config();
    let main_server = MainServer::new();
    main_server.run().await
}
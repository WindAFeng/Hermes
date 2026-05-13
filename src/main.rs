use models::hermes_model::hermes_error::HermesError;
use crate::cache::hermes_cache::HermesCache;
use crate::database_connect_manger::database_manager::DatabaseManager;
use crate::server::server::MainServer;
use crate::utils::{config::init_config, log};
use crate::utils::config::get_config;

pub mod models;
pub mod utils;
pub mod server;
pub mod command_executor;
pub mod database_connect_manger;
pub mod cache;

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    log::debug("Welcome to Hermes system");
    log::debug("Hermes Server started");
    init_config();
    HermesCache::init()?;
    DatabaseManager::init(get_config()).await?;
    let main_server = MainServer::new();
    main_server.run().await
}
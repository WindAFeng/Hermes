use tracing::info;
use crate::cache::hermes_cache::HermesCache;
use crate::database_connect_manger::database_manager::DatabaseManager;
use crate::server::server::MainServer;
use crate::utils::config::get_config;
use crate::utils::config::init_config;
use models::hermes_model::hermes_error::HermesError;

pub mod cache;
pub mod database_cmd_builder;
pub mod database_connect_manger;
pub mod instruction_processing_center;
pub mod models;
pub mod server;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    tracing_subscriber::fmt().with_file(false).init();
    info!("Welcome to Hermes system");
    info!("Hermes Server started");
    init_config();
    HermesCache::init()?;
    info!("Hermes Cache initialized");
    DatabaseManager::init(get_config()).await?;
    info!("Hermes DatabaseManager initialized");
    let main_server = MainServer::new();
    main_server.run().await
}

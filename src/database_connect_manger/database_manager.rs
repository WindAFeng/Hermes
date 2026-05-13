use crate::database_connect_manger::redis_manager::RedisManager;
use crate::models::config::Config;
use crate::models::hermes_model::hermes_error::HermesError;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub static GLOBAL_DB_MANAGER: OnceCell<Arc<DatabaseManager>> = OnceCell::const_new();

pub struct DatabaseManager {
    redis_manager: Arc<RedisManager>,
}
impl DatabaseManager {
    pub async fn init(config: Arc<Config>) -> Result<(), HermesError> {
        let redis_manager = init_redis_pool(config).await?;
        let manager = Arc::new(DatabaseManager {
            redis_manager: Arc::new(redis_manager),
        });
        GLOBAL_DB_MANAGER
            .set(manager)
            .map_err(|_| HermesError::Internal("Failed to set global DB manager".into()))?;
        Ok(())
    }
    pub fn global() -> &'static Arc<DatabaseManager> {
        GLOBAL_DB_MANAGER.get().unwrap()
    }
    pub fn get_redis_pool(&self, db_name: &str) -> Result<Arc<Pool<RedisConnectionManager>>, HermesError> {
        self.redis_manager.get_redis_pool(db_name)
    }
}
async fn init_redis_pool(config: Arc<Config>) -> Result<RedisManager, HermesError> {
    let mut redis_manager = RedisManager::new();
    for (db_name, cfg) in config.database.redis.iter() {
        redis_manager.add_redis_db(db_name, cfg).await?;
    }
    Ok(redis_manager)
}

use crate::models::config::RedisConfig;
use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::utils::db_info_manager::DBInfoManager;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use dashmap::DashMap;
use std::sync::Arc;

struct RedisPoolManager {
    pub pool: Arc<Pool<RedisConnectionManager>>,
    pub priority: u8,
}
pub struct RedisManager {
    pool_manager: DashMap<String, RedisPoolManager>
}
impl RedisManager {
    pub fn new() -> Self {
        Self {
            pool_manager: DashMap::new()
        }
    }
    pub async fn add_redis_db(
        &mut self,
        db_name: &str,
        redis_config: &RedisConfig,
    ) -> Result<(), HermesError> {
        let info = DBInfoManager::new(DatabaseType::Redis, redis_config);
        let (host, port, user, password, db, priority) = (
            info.host(),
            info.port(),
            info.user(),
            info.password(),
            info.db(),
            info.priority(),
        );
        let redis_url = format!("redis://{}:{}@{}:{}/{}", user, password, host, port, db);
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager).await?;
        self.pool_manager.insert(
            db_name.to_string(),
            RedisPoolManager {
                pool: Arc::new(pool),
                priority,
            },
        );
        Ok(())
    }
    pub fn get_redis_pool(&self, db_name: &str) -> Result<Arc<Pool<RedisConnectionManager>>, HermesError> {
        if self.pool_manager.is_empty() {
            return Err(HermesError::Internal("No Redis databases configured".to_string()));
        }
        match self.pool_manager.get(db_name) {
            Some(manager) => Ok(Arc::clone(&manager.pool)),
            None => {
                Err(HermesError::Internal(format!(
                    "Redis database '{}' not found",
                    db_name
                )))
            }
        }
    }
}
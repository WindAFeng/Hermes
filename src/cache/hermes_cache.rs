use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::log::info;

pub static GLOBAL_CACHE: OnceCell<Arc<HermesCache>> = OnceCell::const_new();
pub struct HermesCache {
    cache: Cache<String, HermesType>,
}
impl HermesCache {
    pub fn init() -> Result<(), HermesError> {
        info!("initializing Hermes Cache");
        let cache: Cache<String, HermesType> = Cache::builder()
            .max_capacity(1000)
            .time_to_idle(Duration::from_mins(5))
            .build();
        GLOBAL_CACHE
            .set(Arc::new(HermesCache { cache }))
            .map_err(|err| HermesError::Internal(err.to_string()))
    }
    pub fn global() -> &'static Arc<HermesCache> {
        GLOBAL_CACHE.get().unwrap()
    }
    pub async fn insert(
        &self,
        db_type: &DatabaseType,
        db_name: &str,
        key: &str,
        value: &HermesType,
    ) -> Result<(), HermesError> {
        self.cache
            .insert(
                format!("{}:{}:{}", db_type.to_string(), db_name, key),
                value.clone()
            )
            .await;
        Ok(())
    }
    pub async fn get(&self, key: &str) -> Option<HermesType> {
        self.cache.get(key).await
    }
    pub fn is_empty(&self) -> bool {
        self.cache.entry_count() == 0
    }
    pub async fn remove(&self, key: &str) -> Option<HermesType> {
        self.cache.remove(key).await
    }
    pub async fn delete(&self, key: &str) {
        self.cache.invalidate(key).await;
    }
}

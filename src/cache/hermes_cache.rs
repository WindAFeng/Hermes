use crate::models::cache_value::CacheValue;
use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;

pub static GLOBAL_CACHE: OnceCell<Arc<HermesCache>> = OnceCell::const_new();
pub struct HermesCache {
    cache: Cache<String, CacheValue>,
}
impl HermesCache {
    pub fn init() -> Result<(), HermesError> {
        let cache: Cache<String, CacheValue> = Cache::builder()
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
        &mut self,
        db_type: DatabaseType,
        key: &str,
        value: HermesType,
        value_type: &str,
    ) -> Result<(), HermesError> {
        self.cache
            .insert(
                format!("{}:{}", db_type.to_string(), key),
                CacheValue {
                    value,
                    type_: value_type.to_string(),
                },
            )
            .await;
        Ok(())
    }
    pub async fn get(&self, key: &str) -> Option<CacheValue> {
        self.cache.get(key).await
    }
    pub async fn remove(&mut self, key: &str) -> Option<CacheValue> {
        self.cache.remove(key).await
    }
    pub async fn delete(&mut self, key: &str) {
        self.cache.invalidate(key).await;
    }
}

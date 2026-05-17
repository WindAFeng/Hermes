use crate::cache::hermes_cache::HermesCache;
use crate::instruction_processing_center::adapter::database_adapt::DatabaseAdapt;
use crate::models::database_args_model::RedisArgs;
use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request::Request;
use crate::models::ingest_model::request_model::request_data_value::RequestDataValue;
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CommandHandler {
    adapter: Arc<dyn DatabaseAdapt>,
    request: Request,
}
impl CommandHandler {
    pub fn new(adapter: Arc<dyn DatabaseAdapt>, request: &Request) -> CommandHandler {
        CommandHandler {
            adapter,
            request: request.clone(),
        }
    }
    pub async fn add(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        self.adapter.add().await?;
        let data = self.request.data.as_ref().unwrap();
        let cache = HermesCache::global();
        for (key, RequestDataValue { data, .. }) in data.iter() {
            cache
                .insert(
                    &self.request.database.to_db_type(),
                    &self.request.db_name,
                    key,
                    data,
                )
                .await?;
        }
        Ok(None)
    }

    pub async fn get(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        let db_type = self.request.database.to_db_type();
        // 1. 提取 keys（仅 Redis 支持批量 Get）
        let keys = get_keys(&db_type, &self.request)?;
        if keys.is_empty() {
            return Ok(Some(HashMap::new())); // 空查询返回空 map
        }
        let cache_futures: Vec<_> = keys
            .iter()
            .map(|key| {
                let cache_key = format!(
                    "{}:{}:{}",
                    db_type.to_string(),
                    self.request.db_name, // 避免 clone，&str 足够
                    key
                );
                let key_owned = key.clone();
                async move {
                    let value_opt = HermesCache::global().get(&cache_key).await;
                    (key_owned, value_opt)
                }
            })
            .collect();

        let cache_results: Vec<(String, Option<HermesType>)> = join_all(cache_futures).await;
        let mut result_map: HashMap<String, HermesType> = HashMap::with_capacity(keys.len());
        let mut missing_keys = Vec::new();

        for (key, opt_value) in cache_results {
            if let Some(cache_val) = opt_value {
                result_map.insert(key, cache_val.clone());
            } else {
                missing_keys.push(key);
            }
        }
        if !missing_keys.is_empty() {
            if let Some(db_result) = self.adapter.get(missing_keys).await? {
                let cache = HermesCache::global();
                for (k, v) in db_result.iter() {
                    result_map.insert(k.clone(), v.clone());
                    cache.insert(&db_type, &self.request.db_name, k, v).await?;
                }
            }
        }

        Ok(Some(result_map))
    }
    pub async fn delete(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        let db_type = self.request.database.to_db_type();
        let keys = get_keys(&db_type, &self.request)?;
        if keys.is_empty() {
            return Ok(None);
        }
        let cache = HermesCache::global();
        for k in keys.iter() {
            cache.delete(k).await;
        }
        self.adapter.delete().await?;
        Ok(None)
    }
    pub async fn update(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
    pub async fn use_(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
}
fn get_keys(data_type: &DatabaseType, request: &Request) -> Result<Vec<String>, HermesError> {
    match &data_type {
        DatabaseType::Redis => {
            let args = &request.get_args::<RedisArgs>();
            args.keys
                .clone()
                .ok_or_else(|| HermesError::Internal("keys is empty".to_string()))
        }
        _ => Ok(vec![]),
    }
}

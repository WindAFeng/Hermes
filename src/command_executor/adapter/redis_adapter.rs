use crate::command_executor::adapter::database_adapt::DatabaseAdapt;
use crate::database_connect_manger::database_manager::DatabaseManager;
use crate::models::database_args_model::RedisArgs;
use crate::models::database_data_value_model::RedisDataValue;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request::Request;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct RedisAdapter {
    data: HashMap<String, RedisDataValue>,
    args: RedisArgs,
    db_name: String,
}
impl RedisAdapter {
    pub fn new(request: &Request) -> Self {
        Self {
            data: request.get_redis_data(),
            args: request.get_args(),
            db_name: request.db_name.clone(),
        }
    }
}
#[async_trait]
impl DatabaseAdapt for RedisAdapter {
    async fn add(&self) -> Result<(), HermesError> {
        let db_name = self.db_name.clone();
        let conn = DatabaseManager::global().get_redis_pool(&db_name)?;
        todo!()
    }
    async fn get(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }

    async fn delete(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn update(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn use_(&self) -> Result<(), HermesError> {
        todo!()
    }
}

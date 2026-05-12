use std::collections::HashMap;
use async_trait::async_trait;
use crate::command_executor::adapter::database_adapt::DatabaseAdapt;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request_data_value::RequestDataValue;

pub struct MongoDBAdapter{
    data: HashMap<String, RequestDataValue>,
    args: HashMap<String, HermesType>,
}
impl MongoDBAdapter {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            args: HashMap::new(),
        }
    }
}
#[async_trait]
impl DatabaseAdapt for MongoDBAdapter{

    async fn add(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn get(&self) -> Result<Option<HermesType>, HermesError> {
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
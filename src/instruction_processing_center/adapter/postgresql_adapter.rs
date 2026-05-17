use crate::instruction_processing_center::adapter::database_adapt::DatabaseAdapt;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request_data_value::RequestDataValue;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct PostgreSQLAdapter {
    data: HashMap<String, RequestDataValue>,
    args: HashMap<String, HermesType>,
}
impl PostgreSQLAdapter {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            args: HashMap::new(),
        }
    }
}
#[async_trait]
impl DatabaseAdapt for PostgreSQLAdapter {
    async fn add(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn get(&self, keys: Vec<String>) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
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

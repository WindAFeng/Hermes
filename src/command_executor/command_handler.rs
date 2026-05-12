use std::collections::HashMap;
use crate::command_executor::adapter::database_adapt::DatabaseAdapt;
use std::sync::Arc;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;

pub struct CommandHandler {
    adapter: Arc<dyn DatabaseAdapt>,
}
impl CommandHandler {
    pub fn new(adapter: Arc<dyn DatabaseAdapt>) -> CommandHandler {
        CommandHandler { adapter }
    }
    pub async fn add(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        match self.adapter.add().await {
            Ok(_) => Ok(None),
            Err(error) => Err(HermesError::from(error)),
        }
    }
    pub async fn get(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
    pub async fn delete(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
    pub async fn update(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
    pub async fn use_(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        todo!()
    }
}

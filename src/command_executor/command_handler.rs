use crate::command_executor::adapter::database_adapt::DatabaseAdapt;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request::Request;
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

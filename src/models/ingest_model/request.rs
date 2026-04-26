use std::collections::HashMap;
use crate::models::ingest_model::database_type::DatabaseType;
use crate::models::ingest_model::request_assistant::RequestAssistant;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub command: RequestAssistant,
    pub database: DatabaseType,
    pub args: Option<HashMap<String, Value>>,
    pub data: Option<Value>,
}

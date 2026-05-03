use std::collections::HashMap;
use crate::models::ingest_model::request_database_type::RequestDatabaseType;
use crate::models::ingest_model::request_assistant::RequestAssistant;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::errors::HermesError;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::hermes_command_type::HermesCommandType;
use crate::models::ingest_model::ingest_command_type::IngestCommandType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub command: RequestAssistant,
    pub database: RequestDatabaseType,
    pub db_name: Option<String>,
    pub args: Option<HashMap<String, Value>>,
    pub data: Option<Value>
}
impl Request {
    pub fn get_data(&self) -> Result<Option<DataWrapper>, HermesError>{
        match &self.data {
            Some(Value::Object(hashmap)) =>  {
                let hashmap: HashMap<String, Value> =
                    hashmap.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                Ok(Some(DataWrapper::One(hashmap)))
            }
            Some(Value::Array(list)) => {
                let mut result = Vec::with_capacity(list.len());
                for item in list {
                    match item {
                        Value::Object(obj) => {
                            let hashmap: HashMap<String, Value> =
                                obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                            result.push(hashmap);
                        }
                        _ => {
                            return Err(HermesError::Internal("Not Found Data".to_string()));
                        }
                    }
                }
                Ok(Some(DataWrapper::Many(result)))
            }
            _ => {
                Err(HermesError::Internal("Not Found Data".to_string()))
            }
        }
    }
    pub fn args_to_json(&self) -> Result<String, HermesError> {
        Ok(serde_json::to_string(&self.args)?)
    }
    pub fn split(&self) -> IngestCommandType {
        let command = &self.command;
        match command {
            RequestAssistant::Add => IngestCommandType::Database(DatabaseCommandType::Add),
            RequestAssistant::Get => IngestCommandType::Database(DatabaseCommandType::Get),
            RequestAssistant::Update => IngestCommandType::Database(DatabaseCommandType::Update),
            RequestAssistant::Delete => IngestCommandType::Database(DatabaseCommandType::Delete),
            RequestAssistant::Use => IngestCommandType::Database(DatabaseCommandType::Use),
            RequestAssistant::Set => IngestCommandType::Hermes(HermesCommandType::Set),
            RequestAssistant::Clear => IngestCommandType::Hermes(HermesCommandType::Clear),
            RequestAssistant::Config => IngestCommandType::Hermes(HermesCommandType::Config),
        }
    }
}
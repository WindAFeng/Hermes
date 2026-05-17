use crate::models::database_args_model::database_args::DatabaseArgs;
use crate::models::database_data_type_model::redis_data_type::RedisDataType;
use crate::models::database_data_value_model::RedisDataValue;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::commands::database_commands::DatabaseCommands;
use crate::models::ingest_model::commands::hermes_commands::HermesCommands;
use crate::models::ingest_model::commands::ingest_command_type::IngestCommandType;
use crate::models::ingest_model::request_model::request_assistant::RequestAssistant;
use crate::models::ingest_model::request_model::request_assistant::RequestAssistant::{
    Add, Clear, Config, Delete, Get, Set, Update, Use,
};
use crate::models::ingest_model::request_model::request_data_value::RequestDataValue;
use crate::models::ingest_model::request_model::request_database_type::RequestDatabaseType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Request {
    pub command: RequestAssistant,
    pub database: RequestDatabaseType,
    pub db_name: String,
    pub args: Option<HashMap<String, HermesType>>,
    pub data: Option<HashMap<String, RequestDataValue>>,
}
impl Request {
    pub fn get_redis_data(&self) -> HashMap<String, RedisDataValue> {
        let data = match &self.data {
            Some(d) => d,
            None => return HashMap::new(),
        };
        let mut result_data: HashMap<String, RedisDataValue> = HashMap::with_capacity(data.len());
        for (k, v) in data.iter() {
            let data_type = match v.type_to_json() {
                Ok(t) => {
                    match RedisDataType::from_string(&t) { 
                        Ok(d) => d,
                        Err(_) => return HashMap::new(),
                    }
                },
                Err(_) => return HashMap::new(),
            };
            let data_value = RedisDataValue {
                type_: data_type,
                data: v.data.clone(),
            };
            result_data.insert(k.clone(), data_value);
        }
        result_data
    }
    pub fn get_mysql_data(&self) -> HashMap<String, RedisDataValue> {
        todo!()
    }
    pub fn get_mongodb_data(&self) -> HashMap<String, RedisDataValue> {
        todo!()
    }
    pub fn get_postgresql_data(&self) -> HashMap<String, RedisDataValue> {
        todo!()
    }
    pub fn get_args<A: DatabaseArgs>(&self) -> A {
        match &self.args_to_json() {
            Ok(json) => A::from_str(json.as_str()),
            Err(_) => A::from_str(""),
        }
    }
    fn args_to_json(&self) -> Result<String, HermesError> {
        Ok(serde_json::to_string(&self.args)?)
    }
    pub fn split(&self) -> IngestCommandType {
        let command = &self.command;
        match command {
            Add => IngestCommandType::Database(DatabaseCommands::Add),
            Get => IngestCommandType::Database(DatabaseCommands::Get),
            Update => IngestCommandType::Database(DatabaseCommands::Update),
            Delete => IngestCommandType::Database(DatabaseCommands::Delete),
            Use => IngestCommandType::Database(DatabaseCommands::Use),
            Set => IngestCommandType::Hermes(HermesCommands::Set),
            Clear => IngestCommandType::Hermes(HermesCommands::Clear),
            Config => IngestCommandType::Hermes(HermesCommands::Config),
        }
    }
}

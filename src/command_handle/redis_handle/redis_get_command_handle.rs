use crate::command_handle::get_db_name::get_db_name;
use crate::errors::HermesError;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::redis_lib::create_connect::create_connect;
use crate::redis_lib::redis_executes::redis_string_execute::RedisStringExecute;
use crate::utils::config::get_config;
use std::collections::HashMap;
use serde_json::Value;
use crate::command_handle::get_addr::{get_host, get_port};

async fn get_value(key: &str, host: &str, port: &str) -> Result<String, HermesError> {
    let connect = create_connect(host, port).await?;
    let mut redis_commands = RedisStringExecute::new(connect);
    match redis_commands.get(key).await {
        Ok(Some(value)) => Ok(value),
        _ => Err(HermesError::Internal("Key not found".to_string())),
    }
}
async fn get_values(key: &Vec<String>, host: &str, port: &str) -> Result<Vec<String>, HermesError> {
    let connect = create_connect(host, port).await?;
    let mut redis_commands = RedisStringExecute::new(connect);
    match redis_commands.m_get(key.clone()).await {
        Ok(value) => Ok(value),
        _ => Err(HermesError::Internal("Key not found".to_string())),
    }
}
fn to_value(hashmap_list: &Vec<HashMap<String, String>>) -> Vec<HashMap<String, Value>> {
    let mut result: Vec<HashMap<String, Value>> = Vec::new();
    for item in hashmap_list {
        for (key, value) in item {
            let new_value = Value::String(value.to_string());
            let new_key = key.clone();
            result.push(HashMap::from_iter([(new_key, new_value)]));
        }
    }
    result
}
pub async fn redis_get_command_handle(
    database_name: Option<String>,
    args: RedisArgs,
) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = get_db_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = get_host(&db_name, &config, DatabaseTypes::Redis);
    let port = get_port(&db_name, &config, DatabaseTypes::Redis);
    let keys = args.get_keys()?;
    match keys.len() {
        0 => Ok(Response {
            code: ResponseCodeType::NotFoundKey,
            message: ResponseMessageType::Error("Can't found data".to_string()),
            data: None,
        }),
        1 => {
            let key = keys.get(0).unwrap();
            let value: Value = Value::String(get_value(&key, &host, &port).await?);
            let mut result: HashMap<String, Value> = HashMap::new();
            result.insert(key.clone(), value);
            Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: Some(DataWrapper::One(result)),
            })
        }
        _ => {
            let values = get_values(&keys, &host, &port).await?;
            let result: Vec<HashMap<String, String>> = keys
                .into_iter()
                .zip(values.into_iter())
                .map(|(k, v)| {
                    let mut map = HashMap::new();
                    map.insert(k, v);
                    map
                })
                .collect();
            Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: Some(DataWrapper::Many(to_value(&result))),
            })
        }
    }
}

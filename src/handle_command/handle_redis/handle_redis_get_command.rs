use crate::handle_command::resolve_db_addr::{resolve_database_host, resolve_database_port};
use crate::handle_command::resolve_db_name::resolve_database_name;
use crate::errors::HermesError;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::handle_redis_model::redis_handle_args_model::RedisHandleArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::database_lib::redis_lib::create_connect::establish_redis_connection;
use crate::database_lib::redis_lib::redis_operations::redis_string_operations::RedisStringOperations;
use crate::utils::config::get_config;
use serde_json::Value;
use std::collections::HashMap;

async fn fetch_single_key(key: &str, host: &str, port: &str) -> Result<String, HermesError> {
    let connect = establish_redis_connection(host, port).await?;
    let mut redis_commands = RedisStringOperations::new(connect);
    match redis_commands.get(key).await {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(HermesError::KeyNotFound(key.to_string())),
        Err(e) => Err(e), // 保留原始错误
    }
}
async fn fetch_multiple_keys(key: &Vec<String>, host: &str, port: &str) -> Result<Vec<String>, HermesError> {
    let connect = establish_redis_connection(host, port).await?;
    let mut redis_commands = RedisStringOperations::new(connect);
    match redis_commands.m_get(key.clone()).await {
        Ok(value) => Ok(value),
        _ => Err(HermesError::Internal("Key not found".to_string())),
    }
}
fn convert_to_json_value(hashmap_list: &Vec<HashMap<String, String>>) -> Vec<HashMap<String, Value>> {
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
async fn handle_single_key_request(
    keys: &Vec<String>,
    host: &str,
    port: &str,
) -> Result<Response, HermesError> {
    let key = keys.get(0).unwrap();
    let value: Value = Value::String(fetch_single_key(&key, &host, &port).await?);
    let mut result: HashMap<String, Value> = HashMap::new();
    result.insert(key.clone(), value);
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: Some(DataWrapper::One(result)),
    })
}
async fn handle_multiple_keys_request(keys: &Vec<String>, host: &str, port: &str) -> Result<Response, HermesError> {
    let values = fetch_multiple_keys(&keys, &host, &port).await?;
    let result: Vec<HashMap<String, String>> = keys
        .iter()
        .zip(values.iter())
        .map(|(k, v)| {
            let mut map = HashMap::new();
            map.insert(k.clone(), v.clone());
            map
        })
        .collect();
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: Some(DataWrapper::Many(convert_to_json_value(&result))),
    })
}
pub async fn handle_redis_get_command(
    database_name: Option<String>,
    args: RedisHandleArgs,
) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = resolve_database_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = resolve_database_host(&db_name, &config, DatabaseTypes::Redis);
    let port = resolve_database_port(&db_name, &config, DatabaseTypes::Redis);
    let keys = args.get_keys()?;
    match keys.len() {
        0 => Ok(Response {
            code: ResponseCodeType::BadRequest,
            message: ResponseMessageType::Error("Can't found data".to_string()),
            data: None,
        }),
        1 => {
            handle_single_key_request(&keys, &host, &port).await
        }
        _ => {
            handle_multiple_keys_request(&keys, &host, &port).await
        }
    }
}

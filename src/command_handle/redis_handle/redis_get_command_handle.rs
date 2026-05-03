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
use std::sync::Arc;
use serde_json::Value;
use crate::models::config::Config;

fn get_host(db_name: &str, config: &Arc<Config>) -> String {
    config.database.redis.get(db_name).unwrap().host.clone()
}
fn get_post(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name).unwrap().port{
        Some(p) => p.to_string(),
        None => "6379".to_string()
    }
}
async fn get_value(key: &str, host: &str, port: &str) -> Result<String, HermesError> {
    let connect = match create_connect(host, port).await {
        Ok(connect) => connect,
        Err(err) => return Err(HermesError::from(err)),
    };
    let mut redis_commands = RedisStringExecute::new(connect);
    match redis_commands.get(key).await {
        Ok(Some(value)) => Ok(value),
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
    let host = get_host(&db_name, &config);
    let port = get_post(&db_name, &config);
    let keys = match args.get_keys() {
        Ok(keys) => keys,
        Err(e) => {
            return Ok(Response {
                code: ResponseCodeType::ArgNotFound,
                message: ResponseMessageType::Error(e.to_string()),
                data: None,
            });
        }
    };
    let mut data_list: Vec<HashMap<String, String>> = Vec::new();
    for key in keys {
        let value = match get_value(&key, &host, &port).await {
            Ok(value) => value,
            Err(_) => continue,
        };
        let mut data: HashMap<String, String> = HashMap::new();
        data.insert(key, value);
        data_list.push(data);
    }
    match data_list.len() {
        0 => Ok(Response {
            code: ResponseCodeType::NotFoundKey,
            message: ResponseMessageType::Error("Can't found data".to_string()),
            data: None,
        }),
        1 => {
            let data_ls = to_value(&data_list);
            let data = data_ls.get(0).unwrap().clone();
            Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: Some(DataWrapper::One(data)),
            })
        }
        _ => {
            let data = to_value(&data_list);
            Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: Some(DataWrapper::Many(data)),
            })
        }
    }
}

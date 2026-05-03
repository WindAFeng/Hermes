use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use crate::command_handle::get_db_name::get_db_name;
use crate::errors::HermesError;
use crate::models::config::Config;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisArgs;
use crate::models::hermes_types::HermesTypes;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::data_wrapper::DataWrapper::{One, Many};
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::rust_type::RustType;
use crate::redis_lib::create_connect::create_connect;
use crate::redis_lib::redis_executes::{redis_string_execute::RedisStringExecute};
use crate::utils::config::get_config;
fn get_host(db_name: &str, config: &Arc<Config>) -> String {
    config.database.redis.get(db_name).unwrap().host.clone()
}
fn get_post(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name).unwrap().port{
        Some(p) => p.to_string(),
        None => "6379".to_string()
    }
}
async fn default_handle(data_list: Vec<HashMap<String, Value>>, host: &str, port: &str) -> Result<(), HermesError>{
    Ok(for item in data_list {
        for (key, val) in item {
            let value = RustType::from_value(val).to_string();
            let connect = match create_connect(host, port).await {
                Ok(connect) => connect,
                Err(err) => return Err(HermesError::from(err)),
            };
            let mut redis_commands = RedisStringExecute::new(connect);
            redis_commands.set(&key, &value.to_string()).await?
        }
    })
}
pub async fn redis_add_command_handle(database_name: Option<String>, args: RedisArgs, data: DataWrapper) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = get_db_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = get_host(&db_name, &config);
    let port = get_post(&db_name, &config);
    let value_type = match args.get_value_type() {
        Ok(value_type) => value_type,
        Err(e) => return Ok(Response {
            code: ResponseCodeType::ArgNotFound,
            message: ResponseMessageType::Error(e.to_string()),
            data: None,
        })
    };
    let data_list = match data {
        One(d) => vec![d],
        Many(m) => m,
    };
    match value_type {
        HermesTypes::String | HermesTypes::Int | HermesTypes::UInt | HermesTypes::Float => {
            if let Err(err) = default_handle(data_list, &host, &port).await{
                return Ok(Response {
                    code: ResponseCodeType::ArgNotFound,
                    message: ResponseMessageType::Error(err.to_string()),
                    data: None,
                });
            }
            Ok(
                Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Success,
                    data: None,
                }
            )
        }
        HermesTypes::List => {
            todo!()
        },
        HermesTypes::HashMap => todo!(),
        HermesTypes::None => todo!(),
        HermesTypes::Bool => todo!()
    }
}

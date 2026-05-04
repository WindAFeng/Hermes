use std::collections::HashMap;
use serde_json::Value;
use crate::command_handle::get_addr::{get_host, get_port};
use crate::command_handle::get_db_name::get_db_name;
use crate::errors::HermesError;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisHandleArgs;
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
async fn default_handle(data_list: Vec<HashMap<String, Value>>, host: &str, port: &str) -> Result<(), HermesError>{
    let connect = create_connect(host, port).await?;
    let mut redis_commands = RedisStringExecute::new(connect);
    for item in data_list {
        for (key, val) in item {
            let value = RustType::from_value(val).to_string();
            redis_commands.set(&key, &value).await?
        }
    }
    Ok(())
}
pub async fn redis_add_command_handle(database_name: Option<String>, args: RedisHandleArgs, data: DataWrapper) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = get_db_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = get_host(&db_name, &config, DatabaseTypes::Redis);
    let port = get_port(&db_name, &config, DatabaseTypes::Redis);
    let value_type = match args.get_value_type() {
        Ok(value_type) => value_type,
        Err(e) => return Err(HermesError::from(e))
    };
    let data_list = match data {
        One(d) => vec![d],
        Many(m) => m,
    };
    if data_list.is_empty() {
        return Ok(Response {
            code: ResponseCodeType::BadRequest,
            message: ResponseMessageType::Error("No data provided".to_string()),
            data: None,
        });
    }
    match value_type {
        HermesTypes::String | HermesTypes::Int | HermesTypes::UInt | HermesTypes::Float => {
            if let Err(err) = default_handle(data_list, &host, &port).await{
                return Err(err)
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

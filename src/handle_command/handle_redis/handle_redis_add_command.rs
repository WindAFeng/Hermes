use std::collections::HashMap;
use serde_json::Value;
use crate::handle_command::resolve_db_addr::{resolve_database_host, resolve_database_port};
use crate::handle_command::resolve_db_name::resolve_database_name;
use crate::errors::HermesError;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::handle_redis_model::handle_redis_args_model::RedisHandleArgs;
use crate::models::hermes_types::HermesTypes;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::data_wrapper::DataWrapper::{One, Many};
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::database_lib::redis_lib::create_connect::establish_redis_connection;
use crate::database_lib::redis_lib::redis_operations::{redis_string_operations::RedisStringOperations};
use crate::utils::config::get_config;
async fn add_to_string(data_list: &Vec<HashMap<String, Value>>, host: &str, port: &str) -> Result<(), HermesError>{
    let connect = establish_redis_connection(host, port).await?;
    let mut redis_commands = RedisStringOperations::new(connect);
    match data_list.len() {
        0 => unreachable!("数据为空的检查应该在主函数中完成"),
        1 => {
            let data = data_list.get(0).unwrap().clone();
            let (key, value) = data.into_iter().next().unwrap();
            Ok(redis_commands.set(&key, &value.to_string()).await?)
        }
        _ => {
            let result: Vec<String> = data_list
                .iter()
                .flat_map(|hm| {
                    hm.iter()
                        .flat_map(|(k, v)| vec![k.clone(), v.clone().to_string()])
                })
                .collect();
            Ok(redis_commands.m_set(result).await?)
        }
    }
}
pub async fn handle_redis_add_command(database_name: Option<String>, args: RedisHandleArgs, data: DataWrapper) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = resolve_database_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = resolve_database_host(&db_name, &config, DatabaseTypes::Redis);
    let port = resolve_database_port(&db_name, &config, DatabaseTypes::Redis);
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
            if let Err(err) = add_to_string(&data_list, &host, &port).await{
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

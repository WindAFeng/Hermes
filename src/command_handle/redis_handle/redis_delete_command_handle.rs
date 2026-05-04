use std::collections::HashMap;
use serde_json::{Value};
use crate::command_handle::get_addr::{get_host, get_port};
use crate::command_handle::get_db_name::get_db_name;
use crate::errors::HermesError;
use crate::models::database_types::DatabaseTypes;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisHandleArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::redis_lib::create_connect::create_connect;
use crate::redis_lib::redis_executes::redis_key_execute::RedisKeyExecute;
use crate::utils::config::get_config;
async fn delete_key(keys: &Vec<String>, host: &str, port: &str) -> Result<usize, HermesError>{
    let connect = create_connect(host, port).await?;
    let mut redis_execute = RedisKeyExecute::new(connect);
    redis_execute.del(keys).await
}

pub async fn redis_delete_command_handle(database_name: Option<String>, args: RedisHandleArgs) -> Result<Response, HermesError>{
    let config = get_config();
    let db_name = get_db_name(DatabaseTypes::Redis, database_name, &config)?;
    let host = get_host(&db_name, &config, DatabaseTypes::Redis);
    let port = get_port(&db_name, &config, DatabaseTypes::Redis);
    let keys = args.get_keys()?;
    if keys.is_empty() {
        return Ok(Response {
            code: ResponseCodeType::BadRequest,
            message: ResponseMessageType::Error("No keys provided".to_string()),
            data: None,
        });
    }
    let return_delete_data = args.get_ret_del_data();
    if return_delete_data {
        let mut result_map: HashMap<String, Value> = HashMap::new();
        let result: usize = delete_key(&keys, &host, &port).await?;
        result_map.insert("Success Number".to_string(), Value::Number(result.into()));
        Ok(Response {
            code: ResponseCodeType::Success,
            message: ResponseMessageType::Success,
            data: Some(DataWrapper::One(result_map)),
        })
    }
    else {
        let _ = delete_key(&keys, &host, &port).await?;
        Ok(Response {
            code: ResponseCodeType::Success,
            message: ResponseMessageType::Success,
            data: None,
        })
    }
}
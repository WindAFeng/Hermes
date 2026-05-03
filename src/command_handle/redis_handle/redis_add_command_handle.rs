use crate::errors::HermesError;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisArgs;
use crate::models::hermes_types::HermesTypes;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::data_wrapper::DataWrapper::{One, Many};
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::rust_type::RustType;
use crate::redis_lib::create_connect::create_connect;
use crate::redis_lib::redis_execute::RedisExecute;
use crate::utils::config::get_config;

pub async fn redis_add_command_handle(args: RedisArgs, data: DataWrapper, database_name: Option<String>) -> Result<Response, HermesError> {
    let config = get_config();
    let db_name = match database_name {
        Some(db_name) => db_name,
        None => {
            println!("有傻逼不写db_name,开始寻找最高优先级的redis数据库");
            let redis = config.database.redis.clone();
            if redis.is_empty(){
                return Err(HermesError::Internal("Not Found Redis Database".to_string()))
            }
            let important = redis.iter()
                .min_by_key(|(_, config)| config.important)
                .map(|(k, _)| k);
            println!("终于他妈找到了: {:?}", important);
            match important {
                Some(k) => k.clone(),
                None => return Err(HermesError::Internal("Not Found Redis Database".to_string()))
            }
        }
    };
    let host = config.database.redis.get(&db_name).unwrap().host.clone();
    let port = match config.database.redis.get(&db_name).unwrap().port{
        Some(p) => p.to_string(),
        None => "6379".to_string()
    };
    let value_type = match args.value_type {
        Some(value_type) => value_type,
        None => return Ok(
            Response {
                code: ResponseCodeType::ArgNotFound,
                message: ResponseMessageType::Error(HermesError::Internal("Not Found arg 'value_type'".to_string()).to_string()),
                data: None,
            }
        )
    };
    let data_list = match data {
        One(d) => vec![d],
        Many(m) => m,
    };
    match value_type {
        HermesTypes::String | HermesTypes::Integer | HermesTypes::UInt | HermesTypes::Float => {
            for item in data_list {
                for (key, val) in item {
                    let value = RustType::from_value(val).to_string();
                    let connect = match create_connect(&host, &port).await {
                        Ok(connect) => connect,
                        Err(err) => return Ok(
                            Response {
                                code: ResponseCodeType::ArgNotFound,
                                message: ResponseMessageType::Error(err.to_string()),
                                data: None,
                            }
                        ),
                    };
                    let mut redis_commands = RedisExecute::new(connect);
                    redis_commands.set(&key, &value.to_string()).await?;
                }
            }
        }
        HermesTypes::List => {
            
        },
        HermesTypes::HashMap => todo!(),
        HermesTypes::None => todo!(),
        HermesTypes::Bool => todo!()
    }
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}

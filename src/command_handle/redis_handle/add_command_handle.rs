use crate::errors::HermesError;
use crate::models::handle_modle::redis_handle_modle::redis_args_modle::RedisArgs;
use crate::models::hermes_types::HermesTypes;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::data_wrapper::DataWrapper::{One, Many};
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::rust_type::RustType;
use crate::redis_lib::create_connect::create_connect;
use crate::redis_lib::redis_execute::RedisExecute;
pub async fn add_command_handle(args: RedisArgs, data: DataWrapper) -> Result<Response, HermesError> {
    let value_type = match args.type_ {
        Some(type_) => type_,
        None => return Ok(
            Response {
                code: ResponseCodeType::ArgNotFound,
                message: ResponseMessageType::Error(HermesError::Internal("Not Found arg 'type'".to_string()).to_string()),
                data: None,
            }
        )
    };
    let data_list = match data {
        One(d) => vec![d],
        Many(m) => m,
    };
    match value_type {
        HermesTypes::String | HermesTypes::Integer | HermesTypes::UnsignedInteger | HermesTypes::Float => {
            for item in data_list {
                for (key, val) in item {
                    let value = RustType::from_value(val).to_string();
                    let connect = match create_connect().await {
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
        HermesTypes::List => todo!(),
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

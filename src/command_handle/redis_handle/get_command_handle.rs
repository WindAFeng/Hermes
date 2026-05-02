use crate::errors::HermesError;
use crate::models::handle_modle::redis_handle_modle::redis_args_modle::RedisArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;

pub async fn get_command_handle(args: RedisArgs, data: DataWrapper) -> Result<Response, HermesError> {
    let resp = Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    };
    Ok(resp)
}
use crate::errors::HermesError;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;

pub async fn handle_redis_update_command() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Error(String::from("Command not found.")),
        data: None,
    })
}
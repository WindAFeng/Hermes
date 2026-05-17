use std::collections::HashMap;
use crate::models::ingest_model::response_model::response_message_type::ResponseMessageType;
use serde::Serialize;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::response_model::response_code_type::ResponseCodeType;

#[derive(Serialize, Debug)]
pub struct Response {
    pub code: ResponseCodeType,
    pub message: ResponseMessageType,
    pub data: Option<HashMap<String, HermesType>>,
}

impl Response {
    pub fn new(cmd_result: Result<Option<HashMap<String, HermesType>>, HermesError>) -> Self {
        match cmd_result {
            Ok(data) => success(data),
            Err(e) => error(HermesError::from(e)),
        }
    }
    pub fn to_bytes(&self) -> Result<Vec<u8>, HermesError> {
        match serde_json::to_vec(self) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(HermesError::from(e)),
        }
    }
}
fn success(data: Option<HashMap<String, HermesType>>) -> Response {
    Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data,
    }
}
fn error(error: HermesError) -> Response {
    Response {
        code: ResponseCodeType::Error,
        message: ResponseMessageType::Error(error),
        data: None,
    }
}

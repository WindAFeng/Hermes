use serde::{Serialize};
use crate::errors::HermesError;
use crate::models::ingest_model::{data_wrapper::DataWrapper, response_code_type::ResponseCodeType};
use crate::models::ingest_model::response_message_type::{ResponseMessageType};

#[derive(Serialize, Debug)]
pub struct Response {
    pub code: ResponseCodeType,
    pub message: ResponseMessageType,
    pub data: Option<DataWrapper>,
}
impl Response {
    pub fn to_bytes(&self) -> Result<Vec<u8>, HermesError> {
        match serde_json::to_vec(self) {
            Ok(bytes) => Ok(bytes),
            Err(e) => Err(HermesError::from(e)),
        }
    }
}
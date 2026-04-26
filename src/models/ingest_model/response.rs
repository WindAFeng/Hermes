use serde::{Serialize};
use crate::models::ingest_model::{data_wrapper::DataWrapper, response_code_type::ResponseCodeType};
use crate::models::ingest_model::response_message_type::{ResponseMessageType};

#[derive(Serialize, Debug)]
pub struct Response {
    pub code: ResponseCodeType,
    pub message: ResponseMessageType,
    pub data: Option<DataWrapper>,
}

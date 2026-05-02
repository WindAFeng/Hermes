use crate::command_handle::redis_handle::redis_handle::RedisHandle;
use crate::errors::HermesError;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_type::DatabaseType;
use crate::models::ingest_model::request::Request;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use serde_json::Value;
use std::collections::HashMap;

pub struct CommandHandle {
    request: Request,
}
impl CommandHandle {
    pub fn new(request: Request) -> Self {
        CommandHandle { request }
    }
    fn args_to_json(&self) -> Result<String, HermesError> {
        let args = match &self.request.args {
            Some(args) => args,
            None => return Err(HermesError::Internal("Not Found Args".to_string())),
        };
        match serde_json::to_string(&args) {
            Ok(json) => Ok(json),
            Err(error) => Err(HermesError::from(error)),
        }
    }
    fn get_data(&self) -> Result<Option<DataWrapper>, HermesError> {
        let value = &self.request.data;
        match value {
            Some(Value::Object(map)) => {
                let hashmap: HashMap<String, Value> =
                    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                Ok(Some(DataWrapper::One(hashmap)))
            }
            Some(Value::Array(vec)) => {
                let mut result = Vec::with_capacity(vec.len());
                for item in vec {
                    match item {
                        Value::Object(obj) => {
                            let hashmap: HashMap<String, Value> =
                                obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                            result.push(hashmap);
                        }
                        _ => {
                            return Err(HermesError::Internal("Not Found Data".to_string()));
                        }
                    }
                }
                Ok(Some(DataWrapper::Many(result)))
            }
            _ => Err(HermesError::Internal("Wrong Data type".to_string())),
        }
    }
    async fn database_match(&self) -> Result<Response, HermesError> {
        let args = match self.args_to_json() {
            Ok(json) => json,
            Err(error) => return Err(HermesError::from(error)),
        };
        let data = match self.get_data() {
            Ok(data) => data,
            Err(error) => return Err(HermesError::from(error)),
        };
        match &self.request.database {
            DatabaseType::Redis => {
                let redis_handle = RedisHandle::new(
                    self.request.command.clone(),
                    self.request.db_name.clone(),
                    args,
                    data,
                );
                redis_handle.to_response().await
            }
            DatabaseType::MySql => Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: None,
            }),
            DatabaseType::MongoDB => Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: None,
            }),
            DatabaseType::PostgreSQL => Ok(Response {
                code: ResponseCodeType::Success,
                message: ResponseMessageType::Success,
                data: None,
            }),
        }
    }
    pub async fn get(&self) -> Result<Response, HermesError> {
        self.database_match().await
    }
}

use crate::command_handle::redis_handle::add_command_handle::add_command_handle;
use crate::errors::HermesError;
use crate::models::handle_modle::redis_handle_modle::redis_args_modle::RedisArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::response_message_type::{ResponseMessageType};
use crate::models::ingest_model::request_assistant::RequestAssistant;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;

pub struct RedisHandle{
    command: RequestAssistant,
    database_name: Option<String>,
    args: String,
    data: Option<DataWrapper>,
}
impl RedisHandle{
    pub fn new(command: RequestAssistant,database_name: Option<String>, args: String, data: Option<DataWrapper>) -> Self{
        Self {
            command,
            database_name,
            args,
            data,
        }
    }
    fn get_args(&self) -> Result<RedisArgs, HermesError> {
        serde_json::from_str(&self.args).map_err(HermesError::from)
    }
    fn get_data(&self) -> Result<DataWrapper, HermesError>{
        match &self.data { 
            Some(data_type) => Ok(data_type.clone()),
            None => Err(HermesError::Internal("Hermes Error: Not Found Data".to_string())),
        }
    }
    async fn command_match(&self) -> Result<Response, HermesError>{
        let args = match self.get_args(){
            Ok(args) => args,
            Err(e) => return Err(e)
        };
        match &self.command {
            RequestAssistant::Get => {
                Ok(Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Error(String::from("Command not found.")),
                    data: None,
                })
            },
            RequestAssistant::Add => {
                add_command_handle(args, self.get_data()?, self.database_name.clone()).await
            },
            RequestAssistant::Update => {
                Ok(Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Error(String::from("Command not found.")),
                    data: None,
                })
            },
            RequestAssistant::Delete => {
                Ok(Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Error(String::from("Command not found.")),
                    data: None,
                })
            },
        }
    }
    pub async fn to_response(&self) -> Result<Response, HermesError> {
        self.command_match().await
    }
}
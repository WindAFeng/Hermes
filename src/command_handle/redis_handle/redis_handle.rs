use crate::command_handle::redis_handle::redis_add_command_handle::redis_add_command_handle;
use crate::command_handle::redis_handle::redis_get_command_handle::redis_get_command_handle;
use crate::command_handle::redis_handle::redis_update_command_handle::redis_update_command_handle;
use crate::command_handle::redis_handle::redis_delete_command_handle::redis_delete_command_handle;
use crate::command_handle::redis_handle::redis_use_command_handle::redis_use_command_handle;
use crate::errors::HermesError;
use crate::models::handle_modle::redis_handle_modle::redis_handle_args_model::RedisHandleArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::response::Response;

pub struct RedisHandle{
    command: DatabaseCommandType,
    database_name: Option<String>,
    args: String,
    data: Option<DataWrapper>,
}
impl RedisHandle{
    pub fn new(command: DatabaseCommandType,database_name: Option<String>, args: String, data: Option<DataWrapper>) -> Self{
        Self {
            command,
            database_name,
            args,
            data,
        }
    }
    fn get_args(&self) -> Result<RedisHandleArgs, HermesError> {
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
            DatabaseCommandType::Get => {
                redis_get_command_handle(self.database_name.clone(), args).await
            },
            DatabaseCommandType::Add => {
                redis_add_command_handle(self.database_name.clone(), args, self.get_data()?).await
            },
            DatabaseCommandType::Update => {
                redis_update_command_handle().await
            },
            DatabaseCommandType::Delete => {
                redis_delete_command_handle(self.database_name.clone(), args).await
            }
            DatabaseCommandType::Use => {
                redis_use_command_handle().await
            }
        }
    }
    pub async fn to_response(&self) -> Result<Response, HermesError> {
        self.command_match().await
    }
}
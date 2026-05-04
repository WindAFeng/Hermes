use crate::handle_command::handle_redis::handle_redis_add_command::handle_redis_add_command;
use crate::handle_command::handle_redis::handle_redis_get_command::handle_redis_get_command;
use crate::handle_command::handle_redis::handle_redis_update_command::handle_redis_update_command;
use crate::handle_command::handle_redis::handle_redis_delete_command::handle_redis_delete_command;
use crate::handle_command::handle_redis::handle_redis_use_command::handle_redis_use_command;
use crate::errors::HermesError;
use crate::models::handle_modle::handle_redis_model::handle_redis_args_model::RedisHandleArgs;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::response::Response;

pub struct HandleRedis{
    command: DatabaseCommandType,
    database_name: Option<String>,
    args: String,
    data: Option<DataWrapper>,
}
impl HandleRedis{
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
                handle_redis_get_command(self.database_name.clone(), args).await
            },
            DatabaseCommandType::Add => {
                handle_redis_add_command(self.database_name.clone(), args, self.get_data()?).await
            },
            DatabaseCommandType::Update => {
                handle_redis_update_command().await
            },
            DatabaseCommandType::Delete => {
                handle_redis_delete_command(self.database_name.clone(), args).await
            }
            DatabaseCommandType::Use => {
                handle_redis_use_command().await
            }
        }
    }
    pub async fn to_response(&self) -> Result<Response, HermesError> {
        self.command_match().await
    }
}
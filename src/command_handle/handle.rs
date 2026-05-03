use crate::command_handle::redis_handle::redis_handle::RedisHandle;
use crate::errors::HermesError;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::request_database_type::RequestDatabaseType;
use crate::models::ingest_model::ingest_command_type::IngestCommandType;
use crate::models::ingest_model::request::Request;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
async fn redis_handle(command: &DatabaseCommandType, database_name: Option<String>, args: String, data: Option<DataWrapper>) -> Result<Response, HermesError>{
    let redis_handle = RedisHandle::new(
        command.clone(),
        database_name.clone(),
        args,
        data,
    );
    redis_handle.to_response().await
}
async fn mysql_handle() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
async fn postgresql_handle() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
async fn mongodb_handle() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
pub struct CommandHandle {
    request: Request,
}
impl CommandHandle {
    pub fn new(request: Request) -> Self {
        CommandHandle { request }
    }
    async fn database_match(&self, command: &DatabaseCommandType) -> Result<Response, HermesError> {
        let args = self.request.args_to_json()?;
        let data = self.request.get_data()?;
        match &self.request.database {
            RequestDatabaseType::Redis => redis_handle(command, self.request.db_name.clone(), args, data).await,
            RequestDatabaseType::MySql => mysql_handle().await,
            RequestDatabaseType::MongoDB => mongodb_handle().await,
            RequestDatabaseType::PostgreSQL => postgresql_handle().await,
        }
    }

    pub async fn get(&self) -> Result<Response, HermesError> {
        match &self.request.split() {
            IngestCommandType::Hermes(command) => {
                Ok(Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Success,
                    data: None,
                })
            }
            IngestCommandType::Database(command) => {
                self.database_match(command).await
            }
        }
    }
}

use crate::handle_command::handle_redis::handle_redis::HandleRedis;
use crate::errors::HermesError;
use crate::models::ingest_model::data_wrapper::DataWrapper;
use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::request_database_type::RequestDatabaseType;
use crate::models::ingest_model::ingest_command_type::IngestCommandType;
use crate::models::ingest_model::request::Request;
use crate::models::ingest_model::response::Response;
use crate::models::ingest_model::response_code_type::ResponseCodeType;
use crate::models::ingest_model::response_message_type::ResponseMessageType;
async fn handle_redis(command: &DatabaseCommandType, database_name: Option<String>, args: String, data: Option<DataWrapper>) -> Result<Response, HermesError>{
    let redis_handle = HandleRedis::new(
        command.clone(),
        database_name.clone(),
        args,
        data,
    );
    redis_handle.to_response().await
}
async fn handle_mysql() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
async fn handle_postgresql() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
async fn handle_mongodb() -> Result<Response, HermesError>{
    Ok(Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    })
}
pub struct HandleCommand {
    request: Request,
}
impl HandleCommand {
    pub fn new(request: Request) -> Self {
        Self { request }
    }
    async fn database_match(&self, command: &DatabaseCommandType) -> Result<Response, HermesError> {
        let args = self.request.args_to_json()?;
        let data = self.request.get_data()?;
        match &self.request.database {
            RequestDatabaseType::Redis => handle_redis(command, self.request.db_name.clone(), args, data).await,
            RequestDatabaseType::MySql => handle_mysql().await,
            RequestDatabaseType::MongoDB => handle_mongodb().await,
            RequestDatabaseType::PostgreSQL => handle_postgresql().await,
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

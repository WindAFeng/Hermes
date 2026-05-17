use std::collections::HashMap;
use std::sync::Arc;
use crate::instruction_processing_center::adapter::database_adapt::DatabaseAdapt;
use crate::instruction_processing_center::adapter::MongoDBAdapter;
use crate::instruction_processing_center::adapter::MySQLAdapter;
use crate::instruction_processing_center::adapter::PostgreSQLAdapter;
use crate::instruction_processing_center::adapter::RedisAdapter;
use crate::instruction_processing_center::command_handler::CommandHandler;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::ingest_model::commands::database_commands::DatabaseCommands;
use crate::models::ingest_model::commands::ingest_command_type::IngestCommandType;
use crate::models::ingest_model::request_model::request::Request;
use crate::models::ingest_model::request_model::request_database_type::RequestDatabaseType;
use crate::models::hermes_model::hermes_type::HermesType;

pub struct CommandRouter {
    request: Request,
}
impl CommandRouter {
    pub fn new(request: Request) -> Self {
        Self { request }
    }
    async fn match_database(
        &self,
        command: &DatabaseCommands,
    ) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        let db_type = &self.request.database;
        let adapter: Arc<dyn DatabaseAdapt> = match db_type {
            RequestDatabaseType::Redis => Arc::new(RedisAdapter::new(&self.request)),
            RequestDatabaseType::MySQL => Arc::new(MySQLAdapter::new()),
            RequestDatabaseType::PostgreSQL => Arc::new(PostgreSQLAdapter::new()),
            RequestDatabaseType::MongoDB => Arc::new(MongoDBAdapter::new()),
        };
        let handler = CommandHandler::new(adapter, &self.request);
        match command {
            DatabaseCommands::Add => handler.add().await,
            DatabaseCommands::Get => handler.get().await,
            DatabaseCommands::Update => handler.update().await,
            DatabaseCommands::Delete => handler.delete().await,
            DatabaseCommands::Use => handler.use_().await,
        }
    }

    pub async fn get_result(&self) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        match &self.request.split() {
            IngestCommandType::Hermes(command) => Ok(None),
            IngestCommandType::Database(command) => self.match_database(command).await,
        }
    }
}

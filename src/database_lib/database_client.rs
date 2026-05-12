use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::establish_redis_connection::establish_redis_connection;
use crate::database_lib::resolve_db_addr::{resolve_database_host, resolve_database_port};
use crate::database_lib::resolve_db_name::resolve_database_name;
use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;

pub struct DatabaseClient {
    database_type: DatabaseType,
    database_name: Option<String>
}
impl DatabaseClient {
    pub fn new(database_type: DatabaseType, database_name: Option<String>) -> Self {
        Self {
            database_type,
            database_name
        }
    }
    pub async fn get_connection(&self) -> Result<MultiplexedConnection, HermesError> {
        let db_name = match &self.database_name {
            Some(name) => name,
            None => &resolve_database_name(&self.database_type)?,
        };
        let host = resolve_database_host(&db_name, &self.database_type);
        let port = resolve_database_port(&db_name, &self.database_type);
        match &self.database_type { 
            DatabaseType::Redis => establish_redis_connection(&host, &port).await,
            DatabaseType::MongoDB => todo!(),
            DatabaseType::PostgreSQL => todo!(),
            DatabaseType::MySQL => todo!()
        }
    }
}
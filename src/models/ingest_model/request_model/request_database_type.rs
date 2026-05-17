use serde::{Deserializer};
use crate::models::database_type::DatabaseType;
use crate::models::ingest_model::request_model::request_database_type_visitor::RequestDatabaseKindVisitor;

#[derive(Debug, Clone)]
pub enum RequestDatabaseType {
   MySQL,
   MongoDB,
   Redis,
   PostgreSQL,
}
impl<'de> serde::de::Deserialize<'de> for RequestDatabaseType {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
   where
       D: Deserializer<'de>
   {
      deserializer.deserialize_str(RequestDatabaseKindVisitor)
   }
}
impl RequestDatabaseType {
   pub fn to_db_type(&self) -> DatabaseType {
      match &self {
         RequestDatabaseType::MySQL => DatabaseType::MySQL,
         RequestDatabaseType::MongoDB => DatabaseType::MongoDB,
         RequestDatabaseType::Redis => DatabaseType::Redis,
         RequestDatabaseType::PostgreSQL => DatabaseType::PostgreSQL,
      }
   }
}
use serde::{Deserializer};
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
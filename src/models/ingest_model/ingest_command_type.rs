use crate::models::ingest_model::database_command_type::DatabaseCommandType;
use crate::models::ingest_model::hermes_command_type::HermesCommandType;

pub enum IngestCommandType {
    Hermes(HermesCommandType),
    Database(DatabaseCommandType),
}
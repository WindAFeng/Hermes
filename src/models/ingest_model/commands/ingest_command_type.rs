use crate::models::ingest_model::commands::database_commands::DatabaseCommands;
use crate::models::ingest_model::commands::hermes_commands::HermesCommands;

pub enum IngestCommandType {
    Hermes(HermesCommands),
    Database(DatabaseCommands),
}
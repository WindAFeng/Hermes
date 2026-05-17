use serde::{Deserialize, Deserializer};
use crate::models::ingest_model::request_model::request_assistant_visitor::RequestAssistantVisitor;

#[derive(Debug, Clone)]
pub enum RequestAssistant {
    Add,
    Delete,
    Update,
    Get,
    Use,
    Config,
    Clear,
    Set
}
impl<'de> Deserialize<'de> for RequestAssistant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(RequestAssistantVisitor)
    }
}
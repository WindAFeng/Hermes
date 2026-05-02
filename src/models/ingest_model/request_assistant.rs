use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RequestAssistant {
    Add,
    Delete,
    Update,
    Get,
}
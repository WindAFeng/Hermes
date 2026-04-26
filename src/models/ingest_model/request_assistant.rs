use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestAssistant {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "GET")]
    Get,
}
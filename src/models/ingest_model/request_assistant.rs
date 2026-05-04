use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RequestAssistant {
    Add,
    Del,
    Upd,
    Get,
    Use,
    Cfg,
    Clr,
    Set
}
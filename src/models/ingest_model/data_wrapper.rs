use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataWrapper {
    One(HashMap<String, Value>),
    Many(Vec<HashMap<String, Value>>),
}
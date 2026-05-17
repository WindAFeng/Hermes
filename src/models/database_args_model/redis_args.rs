use std::collections::HashMap;
use serde::Deserialize;
use crate::models::database_args_model::database_args::DatabaseArgs;

#[derive(Default, Deserialize)]
pub struct RedisArgs{
    #[serde(default)]
    pub keys: Option<Vec<String>>,
    pub fields: Option<HashMap<String, Vec<String>>>,
}
impl DatabaseArgs for RedisArgs {
    fn from_str(args: &str) -> Self {
        serde_json::from_str(args).unwrap_or_else(|_| Self::default())
    }
}
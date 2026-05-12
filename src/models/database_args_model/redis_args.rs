use serde::Deserialize;
use crate::models::database_args_model::database_args::DatabaseArgs;
use crate::models::database_model::redis_model::redis_list_push_mode::RedisListPushMode;

#[derive(Default, Deserialize)]
pub struct RedisArgs{
    #[serde(default)]
    pub keys: Option<Vec<String>>,
    pub push_type: Option<RedisListPushMode,>
}
impl DatabaseArgs for RedisArgs {
    fn from_str(args: &str) -> Self {
        serde_json::from_str(args).unwrap_or_else(|_| Self::default())
    }
}
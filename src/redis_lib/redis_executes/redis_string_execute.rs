use redis::aio::MultiplexedConnection;
use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::redis_lib::redis_execute::RedisExecute;
pub struct RedisStringExecute {
    exe: RedisExecute,
}
impl RedisStringExecute {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self { exe: RedisExecute::new(connect) }
    }
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), HermesError> {
        self.exe.execute(
            RedisCommands::SET,
            RedisValueFormat::Default(key.to_string(), value.to_string()),
        )
            .await
    }
    pub async fn get(&mut self, key: &str) -> Result<Option<String>, HermesError> {
        self.exe.execute::<Option<String>>(
            RedisCommands::GET,
            RedisValueFormat::OnlyKey(key.to_string()),
        ).await
    }
}
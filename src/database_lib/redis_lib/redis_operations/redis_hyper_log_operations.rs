use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;

pub struct RedisHyperLogOperations {
    exe: ExecuteRedisCommand
}
impl RedisHyperLogOperations {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { exe: ExecuteRedisCommand::new(connection) }
    }
    pub async fn pf_add(&mut self, key: &str, elements: &[String]) -> Result<(), HermesError> {
        self.exe.execute(RedisCommands::PFADD, RedisValueFormat::List(key.to_string(), elements.to_vec())).await
    }
    pub async fn pf_count(&mut self, keys: &[String]) -> Result<i64, HermesError> {
        self.exe.execute::<i64>(RedisCommands::PFCOUNT, RedisValueFormat::Items(keys.to_vec())).await
    }
    pub async fn pg_merge(&mut self, key1: &[String]) -> Result<(), HermesError> {
        self.exe.execute(RedisCommands::PGMERGE, RedisValueFormat::Items(key1.to_vec())).await
    }
}
use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use redis::aio::MultiplexedConnection;

pub struct RedisStringOperations {
    exe: ExecuteRedisCommand,
}
impl RedisStringOperations {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self {
            exe: ExecuteRedisCommand::new(connect),
        }
    }
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), HermesError> {
        self.exe
            .execute(
                RedisCommands::SET,
                RedisValueFormat::Default(key.to_string(), value.to_string()),
            )
            .await
    }
    pub async fn get(&mut self, key: &str) -> Result<Option<String>, HermesError> {
        self.exe
            .execute::<Option<String>>(
                RedisCommands::GET,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn get_range(
        &mut self,
        key: &str,
        start: i64,
        stop: i64,
    ) -> Result<String, HermesError> {
        self.exe
            .execute(
                RedisCommands::GETRANGE,
                RedisValueFormat::Items(vec![key.to_string(), start.to_string(), stop.to_string()]),
            )
            .await
    }
    pub async fn get_set(
        &mut self,
        key: &str,
        new_value: &str,
    ) -> Result<Option<String>, HermesError> {
        self.exe
            .execute(
                RedisCommands::GETSET,
                RedisValueFormat::Default(key.to_string(), new_value.to_string()),
            )
            .await
    }
    pub async fn get_bit(&mut self, key: &str, offset: i64) {
        todo!()
    }
    pub async fn m_get(&mut self, key: Vec<String>) -> Result<Vec<String>, HermesError> {
        self.exe
            .execute::<Vec<String>>(RedisCommands::MGET, RedisValueFormat::Items(key))
            .await
    }
    pub async fn m_set(&mut self, data_list: Vec<String>) -> Result<(), HermesError> {
        self.exe
            .execute(RedisCommands::MSET, RedisValueFormat::Items(data_list))
            .await
    }
}

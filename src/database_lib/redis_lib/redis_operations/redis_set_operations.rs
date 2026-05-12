use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;

pub struct RedisSetOperations{
    exe: ExecuteRedisCommand
}
impl RedisSetOperations {
    pub fn new(connect: MultiplexedConnection) -> Self{
        Self { exe: ExecuteRedisCommand::new(connect) }
    }
    pub async fn s_add(&mut self, key: &str, items: &[String]) -> Result<(), HermesError> {
        self.exe.execute(RedisCommands::SADD, RedisValueFormat::List(key.to_string(), items.to_vec())).await
    }
    pub async fn s_card(&mut self, keys: &[String]) -> Result<Vec<usize>, HermesError> {
        match keys.len() {
            0 => Ok(vec![0]),
            1 => {
                let key = keys[0].clone();
                let result = self.exe.execute::<usize>(RedisCommands::SCARD, RedisValueFormat::OnlyKey(key.to_string())).await?;
                Ok(vec![result])
            }
            _ => {
                let mut commands: Vec<(RedisCommands, RedisValueFormat)> = Vec::with_capacity(keys.len());
                for key in keys {
                    commands.push((RedisCommands::SCARD, RedisValueFormat::OnlyKey(key.to_string())));
                }
                self.exe.pipe_exec::<Vec<usize>>(commands).await
            }
        }
    }
    pub async fn s_is_member(&mut self, key: &str, member: &str) -> Result<bool, HermesError> {
        self.exe.execute::<bool>(RedisCommands::SISMEMBER, RedisValueFormat::Default(key.to_string(), member.to_string())).await
    }
    pub async fn s_members(&mut self, key: &str) -> Result<Vec<String>, HermesError> {
        self.exe.execute::<Vec<String>>(RedisCommands::SMEMBERS, RedisValueFormat::OnlyKey(key.to_string())).await
    }
    pub async fn s_rem(&mut self, key: &str, member: &[String]) -> Result<usize, HermesError> {
        self.exe.execute::<usize>(RedisCommands::SREM, RedisValueFormat::List(key.to_string(), member.to_vec())).await
    }
}
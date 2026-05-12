use std::collections::HashMap;
use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;

pub struct RedisSortedSetOperations {
    exe: ExecuteRedisCommand,
}
impl RedisSortedSetOperations {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self {
            exe: ExecuteRedisCommand::new(connect),
        }
    }
    pub async fn z_add(&mut self, key: &str, items: HashMap<String, i64>) -> Result<(), HermesError> {
        let mut item_list: Vec<String> = Vec::with_capacity(items.len() * 2);
        for (k, v) in items {
            item_list.push(v.to_string());
            item_list.push(k.to_string());
        }
        self.exe.execute(RedisCommands::ZADD, RedisValueFormat::List(key.to_string(), item_list)).await
    }
    pub async fn z_card(&mut self, key: &str) -> Result<usize, HermesError> {
        self.exe.execute(RedisCommands::ZCARD, RedisValueFormat::OnlyKey(key.to_string())).await
    }
    pub async fn z_rem(&mut self, key: &str, members: &[String]) -> Result<usize, HermesError> {
        let success_num = self.exe.execute::<usize>(RedisCommands::ZREM, RedisValueFormat::List(key.to_string(), members.to_vec())).await?;
        Ok(members.len() - success_num)
    }
    pub async fn z_score(&mut self, key: &str, member: &str) -> Result<i64, HermesError> {
        self.exe.execute::<i64>(RedisCommands::ZSCORE, RedisValueFormat::Default(key.to_string(), member.to_string())).await
    }
}
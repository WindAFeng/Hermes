use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_item_orientation::RedisItemOrientation;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_list_push_mode::RedisListPushMode;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use redis::aio::MultiplexedConnection;

pub struct RedisListOperations {
    exe: ExecuteRedisCommand,
}
impl RedisListOperations {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self {
            exe: ExecuteRedisCommand::new(connect),
        }
    }
    pub async fn l_index(&mut self, key: &str) -> Result<String, HermesError> {
        self.exe
            .execute(
                RedisCommands::LINDEX,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn l_insert(
        &mut self,
        key: &str,
        item_orientation: RedisItemOrientation,
        target_item: &str,
        new_item: &str,
    ) -> Result<(), HermesError> {
        match item_orientation {
            RedisItemOrientation::After => {
                self.exe
                    .execute(
                        RedisCommands::LINSERT,
                        RedisValueFormat::Items(vec![
                            key.to_string(),
                            item_orientation.to_string(),
                            target_item.to_string(),
                            new_item.to_string(),
                        ]),
                    )
                    .await
            }
            RedisItemOrientation::Before => {
                self.exe
                    .execute(
                        RedisCommands::LINSERT,
                        RedisValueFormat::Items(vec![
                            key.to_string(),
                            item_orientation.to_string(),
                            target_item.to_string(),
                            new_item.to_string(),
                        ]),
                    )
                    .await
            }
        }
    }
    pub async fn l_len(&mut self, key: &str) -> Result<usize, HermesError> {
        self.exe
            .execute::<usize>(
                RedisCommands::LLEN,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn l_pop(&mut self, key: &str) -> Result<(), HermesError> {
        self.exe
            .execute(
                RedisCommands::LPOP,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }

    pub async fn push(
        &mut self,
        key: &str,
        push_mode: &RedisListPushMode,
        items: &[String],
    ) -> Result<(), HermesError> {
        match &push_mode {
            RedisListPushMode::Left => {
                self.exe
                    .execute(
                        RedisCommands::LPUSH,
                        RedisValueFormat::List(key.to_string(), items.to_vec()),
                    )
                    .await
            }
            RedisListPushMode::Right => {
                self.exe
                    .execute(
                        RedisCommands::RPUSH,
                        RedisValueFormat::List(key.to_string(), items.to_vec()),
                    )
                    .await
            }
        }
    }
    pub async fn l_range(
        &mut self,
        key: &str,
        from: i64,
        to: i64,
    ) -> Result<Vec<String>, HermesError> {
        self.exe
            .execute(
                RedisCommands::LRANGE,
                RedisValueFormat::Items(vec![key.to_string(), from.to_string(), to.to_string()]),
            )
            .await
    }
    pub async fn l_rem(&mut self, key: &str, index: i64, item: &str) -> Result<(), HermesError> {
        self.exe
            .execute(
                RedisCommands::LREM,
                RedisValueFormat::Items(vec![key.to_string(), index.to_string(), item.to_string()]),
            )
            .await
    }
    pub async fn l_set(&mut self, key: &str, index: i64, item: &str) -> Result<(), HermesError> {
        self.exe
            .execute(
                RedisCommands::LSET,
                RedisValueFormat::Items(vec![key.to_string(), index.to_string(), item.to_string()]),
            )
            .await
    }
    pub async fn push_x(
        &mut self,
        key: &str,
        push_mode: RedisListPushMode,
        item: &str,
    ) -> Result<(), HermesError> {
        match push_mode {
            RedisListPushMode::Left => {
                self.exe
                    .execute(
                        RedisCommands::LPUSHX,
                        RedisValueFormat::Default(key.to_string(), item.to_string()),
                    )
                    .await
            }
            RedisListPushMode::Right => {
                self.exe
                    .execute(
                        RedisCommands::RPUSHX,
                        RedisValueFormat::Default(key.to_string(), item.to_string()),
                    )
                    .await
            }
        }
    }
}

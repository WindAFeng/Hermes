use std::collections::HashMap;
use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::database_lib::redis_lib::async_command_builder::async_command_builder;
use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::async_pipe_command_builder::async_pipe_command_builder;

pub struct RedisExecute {
    connection: MultiplexedConnection,
}
impl RedisExecute {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }
    pub async fn execute<T: redis::FromRedisValue>(
        &mut self,
        command: RedisCommands,
        redis_args: RedisValueFormat,
    ) -> Result<T, HermesError> {
        // 对异步指令 builder 进行简单封装
        async_command_builder(&mut self.connection, command, redis_args).await
    }
    pub async fn pipe_exec<T: redis::FromRedisValue>(
        &mut self,
        commands: HashMap<RedisCommands, RedisValueFormat>,
    ) -> Result<T, HermesError> {
        async_pipe_command_builder(&mut self.connection, commands).await
    }
}

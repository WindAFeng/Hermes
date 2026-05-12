use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use crate::database_lib::redis_lib::async_command_builder::execute_redis_command;
use redis::aio::MultiplexedConnection;
use crate::database_lib::redis_lib::async_pipe_command_builder::execute_redis_pipeline;

pub struct ExecuteRedisCommand {
    connection: MultiplexedConnection,
}
impl ExecuteRedisCommand {
    pub fn new(connection: MultiplexedConnection) -> Self {
        Self { connection }
    }
    pub async fn execute<T: redis::FromRedisValue>(
        &mut self,
        command: RedisCommands,
        value_format: RedisValueFormat,
    ) -> Result<T, HermesError> {
        // 对异步指令 builder 进行简单封装
        execute_redis_command(&mut self.connection, command, value_format).await
    }
    pub async fn pipe_exec<T: redis::FromRedisValue>(
        &mut self,
        commands: Vec<(RedisCommands, RedisValueFormat)>,
    ) -> Result<T, HermesError> {
        execute_redis_pipeline(&mut self.connection, commands).await
    }
}

use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::redis_lib::async_command_builder::async_command_builder;
use redis::aio::MultiplexedConnection;

pub struct RedisExecute {
    connection: MultiplexedConnection,
}
impl RedisExecute {
    pub fn new(connection: MultiplexedConnection) -> RedisExecute {
        RedisExecute { connection }
    }
    pub async fn execute<T: redis::FromRedisValue>(
        &mut self,
        command: RedisCommands,
        redis_args: RedisValueFormat,
    ) -> Result<T, HermesError> {
        // 对异步指令 builder 进行简单封装
        async_command_builder(&mut self.connection, command, redis_args).await
    }
}

use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use redis::FromRedisValue;
use redis::aio::MultiplexedConnection;

pub async fn execute_redis_command<T>(
    conn: &mut MultiplexedConnection,
    command: RedisCommands,
    value_format: RedisValueFormat,
) -> Result<T, HermesError>
where
    T: FromRedisValue,
{
    value_format.auto_format(&command).query_async(conn).await.map_err(HermesError::from)
}

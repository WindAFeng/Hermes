use crate::errors::HermesError;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::models::redis_model::redis_commands::RedisCommands;
use redis::FromRedisValue;
use redis::aio::MultiplexedConnection;

pub async fn async_command_builder<T>(
    conn: &mut MultiplexedConnection,
    command: RedisCommands,
    redis_args: RedisValueFormat,
) -> Result<T, HermesError>
where
    T: FromRedisValue,
{
    let cmd = redis_args.auto_format(&command);
    cmd.query_async(conn).await.map_err(HermesError::from)
}

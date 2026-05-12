use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use redis::{FromRedisValue, Pipeline};
use redis::aio::MultiplexedConnection;
pub async fn execute_redis_pipeline<T>(
    conn: &mut MultiplexedConnection,
    commands: Vec<(RedisCommands, RedisValueFormat)>,
) -> Result<T, HermesError>
where
    T: FromRedisValue,
{
    let mut pipe = Pipeline::new();
    for (command, v_fmt) in commands {
        let cmd = v_fmt.auto_format(&command);
        pipe.add_command(cmd);
    }
    pipe.query_async(conn).await.map_err(HermesError::from)
}

use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use redis::{FromRedisValue, Pipeline};
use redis::aio::MultiplexedConnection;
use std::collections::HashMap;
pub async fn async_pipe_command_builder<T>(
    conn: &mut MultiplexedConnection,
    commands: HashMap<RedisCommands, RedisValueFormat>,
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

use crate::errors::HermesError;
use crate::models::redis_model::redis_argument::RedisArgument;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::utils::log;
use redis::FromRedisValue;
use redis::aio::MultiplexedConnection;

pub async fn async_command_builder<T>(
    conn: &mut MultiplexedConnection,
    command: RedisCommands,
    redis_args: RedisArgument,
) -> Result<T, HermesError>
where
    T: FromRedisValue,
{
    let mut cmd = redis::cmd(command.as_str());
    match redis_args {
        RedisArgument::Default(key, value) => {
            cmd.arg(key).arg(value);
        }
        RedisArgument::List(key, list) => {
            cmd.arg(key);
            for item in list {
                cmd.arg(item);
            }
        }
        RedisArgument::HashMap(key, map) => {
            cmd.arg(key);
            for (ks, vs) in map {
                cmd.arg(ks).arg(vs);
            }
        }
        RedisArgument::OnlyKey(key) => {
            cmd.arg(key);
        },
        RedisArgument::Items(items) => {
            for item in items {
                cmd.arg(item);
            }
        },
        RedisArgument::None => {}
    }

    match cmd.query_async(conn).await {
        Ok(reply) => Ok(reply),
        Err(e) => {
            log::error(format!("Redis command failed: {}", e));
            Err(HermesError::from(e))
        }
    }
}

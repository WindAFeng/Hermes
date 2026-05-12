use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;
use crate::models::hermes_model::hermes_error::HermesError;
use redis::Value;
use redis::aio::MultiplexedConnection;
use std::collections::HashMap;

pub struct RedisHashOperations {
    exe: ExecuteRedisCommand,
}
impl RedisHashOperations {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self {
            exe: ExecuteRedisCommand::new(connect),
        }
    }
    pub async fn h_del(&mut self, key: &str, obj_keys: &[String]) -> Result<usize, HermesError> {
        let result: usize = self
            .exe
            .execute::<usize>(
                RedisCommands::HDEL,
                RedisValueFormat::List(key.to_string(), obj_keys.to_vec()),
            )
            .await?;
        Ok(obj_keys.len() - result)
    }
    pub async fn h_exists(
        &mut self,
        key: &str,
        obj_keys: &[String],
    ) -> Result<Vec<bool>, HermesError> {
        // 提取公共的转换函数
        fn value_to_bool(v: &Value) -> bool {
            matches!(v, Value::Int(1))
        }
        if obj_keys.is_empty() {
            return Ok(Vec::new());
        }
        if obj_keys.len() == 1 {
            // 单个 key 优化路径
            let result = self
                .exe
                .execute::<Vec<Value>>(
                    RedisCommands::HEXISTS,
                    RedisValueFormat::Default(key.to_string(), obj_keys[0].clone()),
                )
                .await?;
            Ok(vec![value_to_bool(&result[0])])
        } else {
            let mut commands: Vec<(RedisCommands, RedisValueFormat)> =
                Vec::with_capacity(obj_keys.len());
            for k in obj_keys {
                commands.push((
                    RedisCommands::HEXISTS,
                    RedisValueFormat::Default(key.to_string(), k.clone()),
                ))
            }
            let results: Vec<Value> = self.exe.pipe_exec::<Vec<Value>>(commands).await?;
            Ok(results.iter().map(value_to_bool).collect())
        }
    }
    pub async fn h_get(&mut self, key: &str, field: &str) -> Result<Option<String>, HermesError> {
        self.exe
            .execute::<Option<String>>(
                RedisCommands::HGET,
                RedisValueFormat::Default(key.to_string(), field.to_string()),
            )
            .await
    }
    pub async fn h_get_all(&mut self, key: &str) -> Result<HashMap<String, String>, HermesError> {
        self.exe
            .execute::<HashMap<String, String>>(
                RedisCommands::HGETALL,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn h_keys(&mut self, key: &str) -> Result<Vec<String>, HermesError> {
        self.exe
            .execute::<Vec<String>>(
                RedisCommands::HKYES,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn h_len(&mut self, key: &str) -> Result<usize, HermesError> {
        self.exe
            .execute::<usize>(
                RedisCommands::HLEN,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
    pub async fn h_m_get(
        &mut self,
        key: &str,
        field: &[String],
    ) -> Result<Vec<Option<String>>, HermesError> {
        match field.len() {
            0 => Ok(Vec::new()),
            _ => {
                self.exe
                    .execute::<Vec<Option<String>>>(
                        RedisCommands::HMGET,
                        RedisValueFormat::List(key.to_string(), field.to_vec()),
                    )
                    .await
            }
        }
    }

    pub async fn h_set(
        &mut self,
        key: &str,
        map: HashMap<String, String>,
    ) -> Result<(), HermesError> {
        self.exe
            .execute(
                RedisCommands::HSET,
                RedisValueFormat::HashMap(key.to_string(), map),
            )
            .await
    }
    pub async fn h_vals(&mut self, key: &str) -> Result<Vec<String>, HermesError> {
        self.exe
            .execute::<Vec<String>>(
                RedisCommands::HVALS,
                RedisValueFormat::OnlyKey(key.to_string()),
            )
            .await
    }
}

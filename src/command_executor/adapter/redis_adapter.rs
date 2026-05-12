use crate::command_executor::adapter::database_adapt::DatabaseAdapt;
use crate::database_lib::database_client::DatabaseClient;
use crate::database_lib::redis_lib::redis_operations::redis_hash_operations::RedisHashOperations;
use crate::database_lib::redis_lib::redis_operations::redis_hyper_log_operations::RedisHyperLogOperations;
use crate::database_lib::redis_lib::redis_operations::redis_list_operations::RedisListOperations;
use crate::database_lib::redis_lib::redis_operations::redis_set_operations::RedisSetOperations;
use crate::database_lib::redis_lib::redis_operations::redis_sorted_set_operations::RedisSortedSetOperations;
use crate::database_lib::redis_lib::redis_operations::redis_string_operations::RedisStringOperations;
use crate::models::database_args_model::RedisArgs;
use crate::models::database_data_type_model::redis_data_type::RedisDataType;
use crate::models::database_data_value_model::RedisDataValue;
use crate::models::database_model::redis_model::redis_list_push_mode::RedisListPushMode;
use crate::models::database_type::DatabaseType;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request::Request;
use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use std::collections::HashMap;
use crate::database_lib::redis_lib::execute_redis_command::ExecuteRedisCommand;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;
use crate::models::database_model::redis_model::redis_value_format::RedisValueFormat;

pub struct RedisAdapter {
    data: HashMap<String, RedisDataValue>,
    args: RedisArgs,
    db_name: Option<String>,
}
impl RedisAdapter {
    pub fn new(request: &Request) -> Self {
        Self {
            data: request.get_redis_data(),
            args: request.get_args(),
            db_name: request.db_name.clone(),
        }
    }
}
#[async_trait]
impl DatabaseAdapt for RedisAdapter {
    async fn add(&self) -> Result<(), HermesError> {
        let data = &self.data;
        if data.is_empty() {
            return Err(HermesError::Internal("No data provided".to_string()));
        }
        let conn = DatabaseClient::new(DatabaseType::Redis, self.db_name.clone())
            .get_connection()
            .await?;
        if data.len() == 1 {
            let (key, value) = data.iter().next().unwrap();
            return handle_single_add(&self.args, conn, key.as_str(), value).await;
        }
        handle_batch_add(&self.args, conn, data).await
    }
    async fn get(&self) -> Result<Option<HermesType>, HermesError> {
        todo!()
    }

    async fn delete(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn update(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn use_(&self) -> Result<(), HermesError> {
        todo!()
    }
}
async fn handle_single_add(
    args: &RedisArgs,
    connect: MultiplexedConnection,
    key: &str,
    value: &RedisDataValue,
) -> Result<(), HermesError> {
    let RedisDataValue { type_, data } = value;
    match &type_ {
        RedisDataType::String => {
            let value = data.as_string()?;
            RedisStringOperations::new(connect).set(key, value).await
        }
        RedisDataType::Hash => {
            let value = data.redis_hash()?;
            RedisHashOperations::new(connect).h_set(key, value).await
        }
        RedisDataType::List => {
            let value = data.redis_list()?;
            let push_type = args
                .push_type
                .as_ref()
                .unwrap_or_else(|| &RedisListPushMode::Left);
            RedisListOperations::new(connect)
                .push(key, &push_type, &value)
                .await
        }
        RedisDataType::Set => {
            let value = data.redis_set()?;
            RedisSetOperations::new(connect).s_add(key, &value).await
        }
        RedisDataType::SortedSet => {
            let value = data.redis_zset()?;
            RedisSortedSetOperations::new(connect)
                .z_add(key, value)
                .await
        }
        RedisDataType::HyperLogLog => {
            let value = data.redis_list()?;
            RedisHyperLogOperations::new(connect)
                .pf_add(key, &value)
                .await
        }
        _ => todo!(),
    }
}
async fn handle_batch_add(
    args: &RedisArgs,
    connect: MultiplexedConnection,
    data: &HashMap<String, RedisDataValue>,
) -> Result<(), HermesError> {
    let mut pipe: Vec<(RedisCommands, RedisValueFormat)> = Vec::with_capacity(data.len());
    for (key, value) in data {
        let RedisDataValue { type_, data } = value;
        match type_ {
            RedisDataType::String => {
                let value = data.redis_string()?;
                pipe.push((RedisCommands::SET, RedisValueFormat::Default(key.to_owned(), value)))
            }
            RedisDataType::Hash => {
                let value = data.redis_hash()?;
                pipe.push((RedisCommands::HSET, RedisValueFormat::HashMap(key.to_owned(), value)))
            }
            RedisDataType::List => {
                let value = data.redis_list()?;
                let push_type = match args.push_type.as_ref(){
                    Some(x) => match x {
                        RedisListPushMode::Left => RedisCommands::LPUSH,
                        RedisListPushMode::Right => RedisCommands::RPUSH,
                    },
                    None => RedisCommands::LPUSH,
                };
                pipe.push((push_type, RedisValueFormat::List(key.to_owned(), value)))
            }
            RedisDataType::Set => {
                let value = data.redis_set()?;
                pipe.push((RedisCommands::SADD, RedisValueFormat::List(key.to_owned(), value)))
            }
            RedisDataType::SortedSet => {
                let value = data.redis_zset()?;
                let mut item_list: Vec<String> = Vec::with_capacity(value.len() * 2);
                for (k, v) in value {
                    item_list.push(v.to_string());
                    item_list.push(k.to_string());
                }
                pipe.push((RedisCommands::ZADD, RedisValueFormat::List(key.to_owned(), item_list)))
            }
            RedisDataType::HyperLogLog => {
                let value = data.redis_list()?;
                pipe.push((RedisCommands::PFADD, RedisValueFormat::List(key.to_owned(), value)))
            }
            _ => todo!()
        }
    }
    ExecuteRedisCommand::new(connect).pipe_exec(pipe).await
}

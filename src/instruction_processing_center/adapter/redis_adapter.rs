use crate::database_cmd_builder::redis_cmd_builder::RedisCmdBuilder;
use crate::database_connect_manger::database_manager::DatabaseManager;
use crate::instruction_processing_center::adapter::database_adapt::DatabaseAdapt;
use crate::models::database_args_model::RedisArgs;
use crate::models::database_data_type_model::redis_data_type::RedisDataType;
use crate::models::database_data_value_model::RedisDataValue;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;
use crate::models::ingest_model::request_model::request::Request;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct RedisAdapter {
    data: HashMap<String, RedisDataValue>,
    args: RedisArgs,
    db_name: String,
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
        if self.data.is_empty() {
            return Err(HermesError::Internal("not found data".to_string()));
        }
        let db_name = &self.db_name;
        let pool = DatabaseManager::global().get_redis_pool(&db_name)?;
        let mut conn = pool.get().await?;
        let mut redis_cmd_builder = RedisCmdBuilder::new();
        let mut string_heap: HashMap<&str, &str> = HashMap::with_capacity(self.data.len());
        let mut hash_heap: HashMap<&str, HashMap<String, String>> =
            HashMap::with_capacity(self.data.len());
        let mut list_heap: HashMap<&str, Vec<String>> = HashMap::with_capacity(self.data.len());
        let mut set_heap: HashMap<&str, Vec<String>> = HashMap::with_capacity(self.data.len());
        let mut zset_heap: HashMap<&str, HashMap<String, String>> =
            HashMap::with_capacity(self.data.len()); // i32 String
        let mut hyperloglog_heap: HashMap<&str, Vec<String>> =
            HashMap::with_capacity(self.data.len());
        for (key, RedisDataValue { type_, data }) in self.data.iter() {
            match type_ {
                RedisDataType::String => {
                    if let HermesType::String(str) = data {
                        string_heap.insert(key, str);
                    }
                }
                RedisDataType::Hash => {
                    if let HermesType::HashMap(hash) = data {
                        let hashmap = to_hash(hash);
                        hash_heap.insert(key, hashmap);
                    }
                }
                RedisDataType::List => {
                    if let HermesType::List(list) = data {
                        let vec = to_vec(list);
                        list_heap.insert(key, vec);
                    }
                }
                RedisDataType::Set => {
                    if let HermesType::List(set) = data {
                        let set = to_vec(set);
                        set_heap.insert(key, set);
                    }
                }
                RedisDataType::SortedSet => {
                    if let HermesType::HashMap(zset) = data {
                        let hashmap = to_zset(zset);
                        zset_heap.insert(key, hashmap);
                    }
                }
                RedisDataType::HyperLogLog => {
                    if let HermesType::List(log) = data {
                        let vec = to_vec(log);
                        hyperloglog_heap.insert(key, vec);
                    }
                },
                &RedisDataType::GEO | &RedisDataType::Stream => todo!()
            }
        }
        if !string_heap.is_empty() {
            redis_cmd_builder.cmd("MSET");
            for (k, v) in string_heap {
                redis_cmd_builder.tuple_arg((k, v));
            }
            redis_cmd_builder.build()?;
        }
        if !hash_heap.is_empty() {
            for (k, v) in hash_heap {
                redis_cmd_builder.cmd("HSET");
                redis_cmd_builder.string_arg(k);
                redis_cmd_builder.hash_arg(v);
                redis_cmd_builder.build()?;
            }
        }
        if !list_heap.is_empty() {
            for (k, v) in list_heap {
                redis_cmd_builder.cmd("RPUSH");
                redis_cmd_builder.string_arg(k);
                redis_cmd_builder.vec_arg(&v);
                redis_cmd_builder.build()?;
            }
        }
        if !set_heap.is_empty() {
            for (k, v) in set_heap {
                redis_cmd_builder.cmd("SADD");
                redis_cmd_builder.string_arg(k);
                redis_cmd_builder.vec_arg(&v);
                redis_cmd_builder.build()?;
            }
        }
        if !zset_heap.is_empty() {
            for (k, v) in zset_heap {
                redis_cmd_builder.cmd("ZADD");
                redis_cmd_builder.string_arg(k);
                redis_cmd_builder.hash_arg(v);
                redis_cmd_builder.build()?;
            }
        }
        if !hyperloglog_heap.is_empty() {
            for (k, v) in hyperloglog_heap {
                redis_cmd_builder.cmd("PFADD");
                redis_cmd_builder.string_arg(k);
                redis_cmd_builder.vec_arg(&v);
                redis_cmd_builder.build()?;
            }
        }
        if redis_cmd_builder.cmd_count() == 1{
            let _ = redis_cmd_builder.to_cmd().query_async::<redis::Value>(&mut *conn).await?;
        } else {
            let _ = redis_cmd_builder.to_pipeline().query_async::<Vec<redis::Value>>(&mut *conn).await?;
        }
        Ok(())
    }
    async fn get(
        &self,
        keys: Vec<String>,
    ) -> Result<Option<HashMap<String, HermesType>>, HermesError> {
        let db_name = &self.db_name;
        let pool = DatabaseManager::global().get_redis_pool(&db_name)?;
        let mut conn = pool.get().await?;
        let mut redis_cmd_builder = RedisCmdBuilder::new();
        for key in keys.iter() {
            redis_cmd_builder.cmd("TYPE").string_arg(key).build()?;
        }
        let key_type_list: Vec<String> = redis_cmd_builder.to_pipeline().query_async(&mut *conn).await?;
        let key_type_pairs: Vec<(&String, RedisDataType)> = key_type_list
            .iter()
            .filter_map(|key| {
                match RedisDataType::from_string(key) {
                    Ok(ty) => Some((key, ty)), // 保留原始 key 和转换结果
                    Err(_) => None,           // 转换失败则丢弃
                }
            })
            .collect();
        for (k, t) in key_type_pairs {
            match &t {
                RedisDataType::String => redis_cmd_builder.cmd("GET").string_arg(k).build()?,
                RedisDataType::Hash => redis_cmd_builder.cmd("HGETALL").string_arg(k).build()?,
                RedisDataType::List => redis_cmd_builder.cmd("LRANGE").string_arg(k).tuple_arg(("0", "-1")).build()?,
                RedisDataType::Set => redis_cmd_builder.cmd("SMEMBERS").string_arg(k).build()?,
                RedisDataType::SortedSet => redis_cmd_builder.cmd("ZRANGE").string_arg(k).tuple_arg(("0", "-1")).string_arg("WITHSCORES").build()?,
                RedisDataType::HyperLogLog => redis_cmd_builder.cmd("PFCOUNT").string_arg(k).build()?,
                RedisDataType::GEO => return Ok(None),
                RedisDataType::Stream => return Ok(None)
            }
        }
        let result_data: HashMap<String, HermesType> = redis_cmd_builder.to_pipeline().query_async(&mut *conn).await?;
        Ok(Some(result_data))
    }

    async fn delete(&self) -> Result<(), HermesError> {
        let db_name = &self.db_name;
        let pool = DatabaseManager::global().get_redis_pool(&db_name)?;
        let mut conn = pool.get().await?;
        let mut redis_cmd_builder = RedisCmdBuilder::new();
        let keys = match &self.args.keys {
            Some(keys) => keys,
            None => return Err(HermesError::Internal("Not Found Arg 'keys'".to_string()))
        };
        for key in keys.iter() {
            redis_cmd_builder.cmd("DEL").string_arg(key).build()?;
        }
        redis_cmd_builder.to_pipeline().query_async(&mut *conn).await.map_err(HermesError::from)
    }

    async fn update(&self) -> Result<(), HermesError> {
        todo!()
    }

    async fn use_(&self) -> Result<(), HermesError> {
        todo!()
    }
}
fn to_hash(hermes_hash: &HashMap<String, HermesType>) -> HashMap<String, String> {
    let mut hm = HashMap::with_capacity(hermes_hash.len());
    for (k, v) in hermes_hash.iter() {
        if let HermesType::String(str) = v {
            hm.insert(k.clone(), str.clone());
        }
    }
    hm
}
fn to_vec(hermes_vec: &Vec<HermesType>) -> Vec<String> {
    let mut vec = Vec::with_capacity(hermes_vec.len());
    for item in hermes_vec.iter() {
        if let HermesType::String(str) = item {
            vec.push(str.clone());
        }
    }
    vec
}
fn to_zset(hermes_zset: &HashMap<String, HermesType>) -> HashMap<String, String> {
    let mut zset = HashMap::with_capacity(hermes_zset.len());
    for (k, v) in hermes_zset.iter() {
        if let HermesType::Integer(int) = v {
            zset.insert(int.to_string().clone(), k.clone());
        }
    }
    zset
}
#[cfg(test)]
mod tests {
    use bb8_redis::RedisConnectionManager;
    use futures_util::future::join_all;
    use redis::cmd;
    use super::*;
    #[tokio::test]
    async fn main() {
        let manager = RedisConnectionManager::new("redis://").unwrap();
        let pool = bb8::Pool::builder().build(manager).await.unwrap();

        let mut handles = vec![];

        for _i in 0..10 {
            let pool = pool.clone();

            handles.push(tokio::spawn(async move {
                let mut conn = pool.get().await.unwrap();

                let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();

                assert_eq!("PONG", reply);
            }));
        }

        join_all(handles).await;
    }
}
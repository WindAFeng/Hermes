use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_key_pattern::RedisKeyPattern;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::redis_lib::async_command_builder::async_command_builder;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use redis::aio::MultiplexedConnection;

pub struct RedisExecute {
    connection: MultiplexedConnection,
}
impl RedisExecute {
    pub fn new(connection: MultiplexedConnection) -> RedisExecute {
        RedisExecute { connection }
    }
    async fn execute<T: redis::FromRedisValue>(
        &mut self,
        command: RedisCommands,
        redis_args: RedisValueFormat,
    ) -> Result<T, HermesError> {
        // 对异步指令 builder 进行简单封装
        async_command_builder(&mut self.connection, command, redis_args).await
    }
    // Key Command
    pub async fn del(&mut self, key: String) -> Result<(), HermesError> {
        // 删除指定 key
        self.execute(RedisCommands::DEL, RedisValueFormat::OnlyKey(key))
            .await
    }
    pub async fn dump(&mut self, key: String) -> Result<String, HermesError> {
        // 返回 base64 格式的字符串
        let dump_bytes: Vec<u8> = match self
            .execute(RedisCommands::DUMP, RedisValueFormat::OnlyKey(key))
            .await
        {
            Ok(bytes) => bytes,
            Err(e) => return Err(e),
        };
        Ok(STANDARD.encode(&dump_bytes))
    }
    pub async fn exists(&mut self, key: String) -> Result<bool, HermesError> {
        // 若 key 存在返回 True, 否则返回 false
        self.execute(RedisCommands::EXISTS, RedisValueFormat::OnlyKey(key))
            .await
    }
    async fn set_expiration_time(
        &mut self,
        command: RedisCommands,
        key: String,
        n: u64,
    ) -> Result<u8, HermesError> {
        let string_u64 = n.to_string();
        self.execute(command, RedisValueFormat::Default(key, string_u64))
            .await
    }
    pub async fn expire(&mut self, key: String, n: u64) -> Result<u8, HermesError> {
        // 设置 key 的过期时间
        // 设置成功返回1,key不存在或失败返回0
        self.set_expiration_time(RedisCommands::EXPIRE, key, n)
            .await
    }
    pub async fn expireat(&mut self, key: String, n: u64) -> Result<u8, HermesError> {
        // 设置 key 的过期时间(Unix时间戳)
        // 设置成功返回1, key 不存在或失败返回0
        self.set_expiration_time(RedisCommands::EXPIREAT, key, n)
            .await
    }
    pub async fn pexpire(&mut self, key: String, n: u64) -> Result<u8, HermesError> {
        // 设置 key 的过期时间亿以毫秒计
        // 设置成功返回1, key 不存在或失败返回0
        self.set_expiration_time(RedisCommands::PEXPIRE, key, n)
            .await
    }
    pub async fn pexpireat(&mut self, key: String, n: u64) -> Result<u8, HermesError> {
        // 设置 key 的过期时间亿以毫秒计(Unix时间戳)
        // 设置成功返回1, key 不存在或失败返回0
        self.set_expiration_time(RedisCommands::PEXPIREAT, key, n)
            .await
    }
    pub async fn keys(&mut self, pattern: RedisKeyPattern) -> Result<Vec<String>, HermesError> {
        let patterns = match pattern {
            RedisKeyPattern::All => vec!["*".to_string()],
            RedisKeyPattern::Keys(keys) => keys,
            RedisKeyPattern::StartFrom(key_start) => vec![format!("{}*", key_start)],
        };
        self.execute(RedisCommands::KEYS, RedisValueFormat::Items(patterns))
            .await
    }
    // String Command
    pub async fn set(&mut self, key: &str, value: &str) -> Result<(), HermesError> {
        self.execute(
            RedisCommands::SET,
            RedisValueFormat::Default(key.to_string(), value.to_string()),
        )
        .await
    }
    pub async fn get(&mut self, key: &str) -> Result<Option<String>, HermesError> {
        self.execute(
            RedisCommands::GET,
            RedisValueFormat::OnlyKey(key.to_string()),
        ).await
    }
}

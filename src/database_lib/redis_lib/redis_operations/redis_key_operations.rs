use crate::errors::HermesError;
use crate::models::redis_model::redis_commands::RedisCommands;
use crate::models::redis_model::redis_key_pattern::RedisKeyPattern;
use crate::models::redis_model::redis_value_format::RedisValueFormat;
use crate::database_lib::redis_lib::redis_execute::RedisExecute;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use redis::aio::MultiplexedConnection;

pub struct RedisKeyOperations {
    exe: RedisExecute,
}
impl RedisKeyOperations {
    pub fn new(connect: MultiplexedConnection) -> Self {
        Self { exe: RedisExecute::new(connect) }
    }
    pub async fn del(&mut self, keys: &[String]) -> Result<usize, HermesError> {
        if keys.is_empty() {
            return Ok(0);
        }
        // 删除指定 key
        let deleted_count: usize = self
            .exe
            .execute::<usize>(
                RedisCommands::DEL,
                RedisValueFormat::Items(keys.to_vec()),
            )
            .await?;
        Ok(deleted_count)
    }
    pub async fn dump(&mut self,key: String) -> Result<String, HermesError> {
        // 返回 base64 格式的字符串
        let dump_bytes: Vec<u8> = match self.exe
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
        self.exe.execute(RedisCommands::EXISTS, RedisValueFormat::OnlyKey(key))
            .await
    }
    async fn set_expiration_time(
        &mut self,
        command: RedisCommands,
        key: String,
        n: u64,
    ) -> Result<u8, HermesError> {
        let string_u64 = n.to_string();
        self.exe.execute(command, RedisValueFormat::Default(key, string_u64))
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
        // 查找所有符合给定模式(pattern)的 key
        let patterns = match pattern {
            RedisKeyPattern::All => "*".to_string(),
            RedisKeyPattern::StartFrom(key_start) => format!("{}*", key_start),
            RedisKeyPattern::EndFrom(key_end) => format!("*{}", key_end),
        };
        self.exe.execute(RedisCommands::KEYS, RedisValueFormat::OnlyKey(patterns))
            .await
    }
}
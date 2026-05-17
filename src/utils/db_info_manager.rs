use crate::models::config::RedisConfig;
use crate::models::database_type::DatabaseType;
pub struct DBInfoManager {
    cached_config: Option<DatabaseConfigRef>,
}

struct DatabaseConfigRef {
    host: String,
    port: String,
    priority: u8,
    database: String,
    user: String,
    password: String,
}

impl DBInfoManager {
    pub fn new(db_type: DatabaseType, redis_config: &RedisConfig) -> Self {
        let cached_config = load_config(redis_config, &db_type);
        Self {
            cached_config,
        }
    }
    
    pub fn host(&self) -> String {
        self.cached_config
            .as_ref()
            .map(|c| c.host.clone())
            .unwrap_or_else(|| "127.0.0.1".to_string())
    }

    pub fn port(&self) -> String {
        self.cached_config
            .as_ref()
            .map(|c| c.port.clone())
            .unwrap_or_else(|| "6379".to_string())
    }

    pub fn priority(&self) -> u8 {
        self.cached_config
            .as_ref()
            .map(|c| c.priority)
            .unwrap_or(0)
    }

    pub fn db(&self) -> String {
        self.cached_config
            .as_ref()
            .map(|c| c.database.clone())
            .unwrap_or_else(|| "0".to_string())
    }

    pub fn user(&self) -> String {
        self.cached_config
            .as_ref()
            .map(|c| c.user.clone())
            .unwrap_or_else(|| "default".to_string()) // Redis ACL 默认用户名是 "default"
    }

    pub fn password(&self) -> String {
        self.cached_config
            .as_ref()
            .map(|c| c.password.clone())
            .unwrap_or_default()
    }
}
fn load_config(
    redis_config: &RedisConfig,
    db_type: &DatabaseType,
) -> Option<DatabaseConfigRef> {
    match db_type {
        DatabaseType::Redis => Some(DatabaseConfigRef {
            host: redis_config.host.clone(),
            port: redis_config
                .port
                .map(|p| p.to_string())
                .unwrap_or_else(|| "6379".to_string()),
            priority: redis_config.priority,
            database: redis_config.database.clone()?,
            user: redis_config.user.clone(),
            password: redis_config.password.clone()?,
        }),
        _ => None,
    }
}

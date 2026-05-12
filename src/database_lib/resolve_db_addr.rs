use std::sync::Arc;
use crate::models::config::Config;
use crate::models::database_type::DatabaseType;
use crate::utils::config::get_config;

fn default_localhost() -> String {
    "127.0.0.1".to_string()
}
fn extract_mysql_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.mysql.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => default_localhost()
    }
}
fn extract_postgres_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.postgres.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => default_localhost()
    }
}
fn extract_redis_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => default_localhost()
    }
}
fn extract_mongodb_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.mongodb.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => default_localhost()
    }
}
pub fn resolve_database_host(db_name: &str, database_types: &DatabaseType) -> String {
    let config = &get_config();
    match database_types {
        DatabaseType::MySQL => extract_mysql_host(db_name, config),
        DatabaseType::PostgreSQL => extract_postgres_host(db_name, config),
        DatabaseType::Redis => extract_redis_host(db_name, config),
        DatabaseType::MongoDB => extract_mongodb_host(db_name, config)
    }
}
fn extract_mysql_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
fn extract_postgres_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
fn extract_redis_port(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name).unwrap().port{
        Some(p) => p.to_string(),
        None => "6379".to_string()
    }
}
fn extract_mongodb_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
pub fn resolve_database_port(db_name: &str, database_types: &DatabaseType) -> String {
    let config = &get_config();
    match database_types {
        DatabaseType::MySQL => extract_mysql_port(db_name, config),
        DatabaseType::PostgreSQL => extract_postgres_port(db_name, config),
        DatabaseType::Redis => extract_redis_port(db_name, config),
        DatabaseType::MongoDB => extract_mongodb_port(db_name, config)
    }
}
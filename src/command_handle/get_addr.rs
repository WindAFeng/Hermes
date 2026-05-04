use std::sync::Arc;
use crate::models::config::Config;
use crate::models::database_types::DatabaseTypes;
fn local_host() -> String {
    "127.0.0.1".to_string()
}
fn get_mysql_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.mysql.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => local_host()
    }
}
fn get_postgres_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.postgres.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => local_host()
    }
}
fn get_redis_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => local_host()
    }
}
fn get_mongodb_host(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.mongodb.get(db_name) {
        Some(cfg) => cfg.host.clone(),
        None => local_host()
    }
}
pub fn get_host(db_name: &str, config: &Arc<Config>, database_types: DatabaseTypes) -> String {
    match database_types {
        DatabaseTypes::MySQL => get_mysql_host(db_name, config),
        DatabaseTypes::PostgreSQl => get_postgres_host(db_name, config),
        DatabaseTypes::Redis => get_redis_host(db_name, config),
        DatabaseTypes::MongoDB => get_mongodb_host(db_name, config)
    }
}
fn get_mysql_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
fn get_postgres_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
fn get_redis_port(db_name: &str, config: &Arc<Config>) -> String {
    match config.database.redis.get(db_name).unwrap().port{
        Some(p) => p.to_string(),
        None => "6379".to_string()
    }
}
fn get_mongodb_port(db_name: &str, config: &Arc<Config>) -> String {
    todo!()
}
pub fn get_port(db_name: &str, config: &Arc<Config>, database_types: DatabaseTypes) -> String {
    match database_types {
        DatabaseTypes::MySQL => get_mysql_port(db_name, config),
        DatabaseTypes::PostgreSQl => get_postgres_port(db_name, config),
        DatabaseTypes::Redis => get_redis_port(db_name, config),
        DatabaseTypes::MongoDB => get_mongodb_port(db_name, config)
    }
}
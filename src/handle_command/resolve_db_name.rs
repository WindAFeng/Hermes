use std::sync::Arc;
use crate::errors::HermesError;
use crate::models::config::{Config, DatabaseConfig};
use crate::models::database_types::DatabaseTypes;
fn select_highest_priority_redis(db: &DatabaseConfig) -> Result<String, HermesError> {
    let redis = db.redis.clone();
    if redis.is_empty(){
        return Err(HermesError::Internal("Not Found Redis Database".to_string()))
    }
    let selected_redis  = redis.iter()
        .min_by_key(|(_, config)| config.important)
        .map(|(k, _)| k);
    selected_redis.cloned().ok_or_else(|| HermesError::Internal("Not Found Redis Database".to_string()))
}
pub fn resolve_database_name(database_type: DatabaseTypes, database_name: Option<String>, config: &Arc<Config>) -> Result<String, HermesError> {
    match database_name {
        Some(db_name) => Ok(db_name),
        None => {
            let db = config.database.clone();
            match database_type {
                DatabaseTypes::Redis => select_highest_priority_redis(&db),
                DatabaseTypes::MongoDB => todo!(),
                DatabaseTypes::MySQL => todo!(),
                DatabaseTypes::PostgreSQl => todo!()
            }
        }
    }
}
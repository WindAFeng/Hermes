use std::sync::Arc;
use crate::errors::HermesError;
use crate::models::config::{Config, DatabaseConfig};
use crate::models::database_types::DatabaseTypes;
fn get_redis_name(db: &DatabaseConfig) -> Result<String, HermesError> {
    let redis = db.redis.clone();
    if redis.is_empty(){
        return Err(HermesError::Internal("Not Found Redis Database".to_string()))
    }
    let important = redis.iter()
        .min_by_key(|(_, config)| config.important)
        .map(|(k, _)| k);
    match important {
        Some(k) => Ok(k.clone()),
        None => return Err(HermesError::Internal("Not Found Redis Database".to_string()))
    }
}
pub fn get_db_name(database_type: DatabaseTypes, database_name: Option<String>, config: &Arc<Config>) -> Result<String, HermesError> {
    match database_name {
        Some(db_name) => Ok(db_name),
        None => {
            let db = config.database.clone();
            match database_type {
                DatabaseTypes::Redis => get_redis_name(&db),
                DatabaseTypes::MongoDB => todo!(),
                DatabaseTypes::MySQL => todo!(),
                DatabaseTypes::PostgreSQl => todo!()
            }
        }
    }
}
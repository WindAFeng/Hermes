use crate::HermesError;
use crate::models::config::DatabaseConfig;
use crate::models::database_type::DatabaseType;
use crate::utils::config::get_config;

fn select_highest_priority_redis(db: &DatabaseConfig) -> Result<String, HermesError> {
    let redis = db.redis.clone();
    if redis.is_empty() {
        return Err(HermesError::Internal(
            "Not Found Redis Database".to_string(),
        ));
    }
    let selected_redis = redis
        .iter()
        .min_by_key(|(_, config)| config.important)
        .map(|(k, _)| k);
    selected_redis
        .cloned()
        .ok_or_else(|| HermesError::Internal("Not Found Redis Database".to_string()))
}
pub fn resolve_database_name(
    database_type: &DatabaseType,
) -> Result<String, HermesError> {
    let config = &get_config();
    let db = config.database.clone();
    match database_type {
        DatabaseType::Redis => select_highest_priority_redis(&db),
        DatabaseType::MongoDB => todo!(),
        DatabaseType::MySQL => todo!(),
        DatabaseType::PostgreSQL => todo!(),
    }
}

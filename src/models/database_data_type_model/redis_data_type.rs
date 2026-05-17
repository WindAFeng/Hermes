use serde::Deserializer;
use crate::models::database_data_type_model::redis_data_type_visitor::RequestDataTypeVisitor;
use crate::models::hermes_model::hermes_error::HermesError;

#[derive(Clone, Debug)]
pub enum RedisDataType {
    String,
    Hash,
    List,
    Set,
    SortedSet,
    HyperLogLog,
    GEO,
    Stream,
}
impl RedisDataType {
    pub fn from_string(json_type: &str) -> Result<Self, HermesError> {
        serde_json::from_str(json_type.to_lowercase().as_str()).map_err(|e| HermesError::Internal(format!("{}", e)))
    }
}
impl<'de> serde::de::Deserialize<'de> for RedisDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(RequestDataTypeVisitor)
    }
}
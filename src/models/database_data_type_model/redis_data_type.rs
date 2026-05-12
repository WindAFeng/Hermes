use serde::Deserializer;
use crate::models::database_data_type_model::redis_data_type_visitor::RequestDataTypeVisitor;

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
    pub fn from_string(json_type: String) -> Self {
        serde_json::from_str(json_type.as_str()).unwrap()
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
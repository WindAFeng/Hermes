use crate::models::database_data_type_model::redis_data_type::RedisDataType;

pub struct RequestDataTypeVisitor;
impl<'de> serde::de::Visitor<'de> for RequestDataTypeVisitor {
    type Value = RedisDataType;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a RequestAssistant (e.g., \"mysql\", \"redis\")")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_lowercase().as_str() {
            "string" => Ok(RedisDataType::String),
            "hash" => Ok(RedisDataType::Hash),
            "list" => Ok(RedisDataType::List),
            "set" => Ok(RedisDataType::Set),
            "zset" => Ok(RedisDataType::SortedSet),
            "hyperloglog" => Ok(RedisDataType::HyperLogLog),
            "geo" => Ok(RedisDataType::GEO),
            "stream" => Ok(RedisDataType::Stream),
            _ => Err(E::unknown_variant(
                value,
                &[
                    "mysql", "redis", "mongodb", "postgresql"
                ],
            )),
        }
    }
}

use crate::models::ingest_model::request_model::request_database_type::RequestDatabaseType;

pub struct RequestDatabaseKindVisitor;
impl<'de> serde::de::Visitor<'de> for RequestDatabaseKindVisitor {
    type Value = RequestDatabaseType;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string representing a RequestAssistant (e.g., \"mysql\", \"redis\")")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.to_lowercase().as_str() {
            "mysql" => Ok(RequestDatabaseType::MySQL),
            "redis" => Ok(RequestDatabaseType::Redis),
            "mongodb" => Ok(RequestDatabaseType::MongoDB),
            "postgresql" => Ok(RequestDatabaseType::PostgreSQL),
            _ => Err(E::unknown_variant(
                value,
                &[
                    "mysql", "redis", "mongodb", "postgresql"
                ],
            )),
        }
    }
}

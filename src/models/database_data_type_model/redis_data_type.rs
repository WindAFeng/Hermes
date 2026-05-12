use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
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

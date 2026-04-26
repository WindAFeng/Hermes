pub enum RedisTypes {
    None,
    String,
    List,
    Set,
    ZSet,
    Hash
}
impl RedisTypes {
    pub fn to_str(&self) -> &'static str {
        match self {
            RedisTypes::None => "none",
            RedisTypes::String => "string",
            RedisTypes::List => "list",
            RedisTypes::Set => "set",
            RedisTypes::ZSet => "zset",
            RedisTypes::Hash => "hash"
        }
    }
    pub fn from_str<T: AsRef<str>>(value: T) -> RedisTypes {
        match value.as_ref() {
            "none" => RedisTypes::None,
            "string" => RedisTypes::String,
            "list" => RedisTypes::List,
            "set" => RedisTypes::Set,
            "zset" => RedisTypes::ZSet,
            "hash" => RedisTypes::Hash,
            _ => RedisTypes::None,
        }
    }
}
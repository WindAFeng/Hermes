pub enum RedisTypes {
    None,
    String,
    List,
    Set,
    ZSet,
    Hash,
    HyperLogLog
}
impl RedisTypes {
    pub fn to_string(&self) -> String {
        let str = match self {
            RedisTypes::None => "none",
            RedisTypes::String => "string",
            RedisTypes::List => "list",
            RedisTypes::Set => "set",
            RedisTypes::ZSet => "zset",
            RedisTypes::Hash => "hash",
            RedisTypes::HyperLogLog => "hyperloglog",
        };
        str.to_owned()
    }
    pub fn from_str<T: AsRef<str>>(value: T) -> RedisTypes {
        match value.as_ref() {
            "string" => RedisTypes::String,
            "list" => RedisTypes::List,
            "set" => RedisTypes::Set,
            "zset" => RedisTypes::ZSet,
            "hash" => RedisTypes::Hash,
            "hyperloglog" => RedisTypes::HyperLogLog,
            _ => RedisTypes::None,
        }
    }
}
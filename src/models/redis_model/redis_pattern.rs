pub enum RedisPattern{
    Keys(Vec<String>),
    All,
    StartFrom(String),
}
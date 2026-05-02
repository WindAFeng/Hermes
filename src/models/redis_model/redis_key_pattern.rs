pub enum RedisKeyPattern{
    Keys(Vec<String>),
    All,
    StartFrom(String),
}
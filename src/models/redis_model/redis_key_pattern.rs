pub enum RedisKeyPattern{
    All,
    StartFrom(String),
    EndFrom(String),
}
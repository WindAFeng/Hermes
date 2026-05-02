use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum RedisCommandType {
    New,
    Append
}
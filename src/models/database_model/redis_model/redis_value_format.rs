use std::collections::HashMap;
use redis::Cmd;
use crate::models::database_model::redis_model::redis_commands::RedisCommands;

pub enum RedisValueFormat {
    Default(String, String),
    List(String, Vec<String>),
    HashMap(String, HashMap<String, String>),
    OnlyKey(String),
    Items(Vec<String>),
    None
}
impl RedisValueFormat {
    pub fn auto_format(&self, command: &RedisCommands) -> Cmd{
        let mut cmd = Cmd::new();
        cmd.arg(command.as_str());
        match &self {
            RedisValueFormat::Default(key, value) => {
                cmd.arg(key).arg(value);
            }
            RedisValueFormat::List(key, list) => {
                cmd.arg(key);
                for item in list {
                    cmd.arg(item);
                }
            }
            RedisValueFormat::HashMap(key, map) => {
                cmd.arg(key);
                for (ks, vs) in map {
                    cmd.arg(ks).arg(vs);
                }
            }
            RedisValueFormat::OnlyKey(key) => {
                cmd.arg(key);
            },
            RedisValueFormat::Items(items) => {
                for item in items {
                    cmd.arg(item);
                }
            },
            RedisValueFormat::None => {}
        }
        cmd
    }
}
use std::collections::HashMap;

pub enum RedisArgument {
    Default(String, String),
    List(String, Vec<String>),
    HashMap(String, HashMap<String, String>),
    OnlyKey(String),
    Items(Vec<String>),
    None
}
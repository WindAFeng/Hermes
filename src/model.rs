use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum MaybeMany {
    One(HashMap<String, Value>),
    Many(Vec<HashMap<String, Value>>),
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Response {
    pub code: u8,
    pub message: String,
    pub data: Option<MaybeMany>,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DataBase {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: Option<u16>,
    pub database_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Request {
    pub command: String,
    pub database: DataBase,
    pub table: String,
    pub args: HashMap<String, Value>,
    pub data: Value,
}
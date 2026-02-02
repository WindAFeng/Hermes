use crate::command_resolution::analytical_structure::{parse_condition, get_command, get_order_by};
use crate::model::{AnalysisArgs, Command, Condition, OrderBy};
use crate::model::{DataBase, MaybeMany, Request};
use serde_json::{Map, Value};
use std::collections::HashMap;
use crate::model::MaybeMany::{Many, One};

#[derive(Debug)]
pub(crate) struct CommandResolution {
    pub database: String,
    pub command: Command,
    pub table: String,
    pub args: AnalysisArgs,
}
impl CommandResolution {
    pub fn new(request: Request) -> CommandResolution {
        let args_ = AnalysisArgs {
            where_logic: init_analysis_args(&request),
            limit: init_limit(&request),
            order_by: init_order_by(&request),
            data: init_data(&request),
        };
        CommandResolution {
            database: init_database_url(&request.database),
            command: get_command(&request.command),
            table: request.table,
            args: args_,
        }
    }
}
fn init_database_url(data_base: &DataBase) -> String {
    let port = data_base.port.unwrap_or(3306);
    format!(
        "mysql://{}:{}@{}:{}/{}",
        data_base.username, data_base.password, data_base.host, port, data_base.database_name
    )
}
fn init_analysis_args(request: &Request) -> Condition {
    request
        .args
        .get("where")
        .and_then(|value| parse_condition(value).ok()) // 解析成功则返回 Some(condition)，失败则返回 None
        .unwrap_or_else(|| Condition::NONE)
}
fn init_limit(request: &Request) -> Option<u32> {
    const MAX_LIMIT: u32 = 1000;
    const DEFAULT_LIMIT: u32 = 20;

    let limit = request
        .args
        .get("limit")
        .and_then(|val| {
            // 假设 val 是 Value 类型，使用方案一的逻辑
            if let Some(u64_val) = val.as_u64() {
                if u64_val <= (u32::MAX as u64) {
                    Some(u64_val as u32)
                } else {
                    None
                }
            } else if let Ok(num) = val.to_string().parse::<u32>() {
                Some(num)
            } else {
                None
            }
        })
        .unwrap_or(DEFAULT_LIMIT); // 如果没有传，默认给 20

    // 强制限制最大值
    Some(limit.min(MAX_LIMIT))
}
fn init_order_by(request: &Request) -> Option<HashMap<String, OrderBy>> {
    match request.args.get("order_by")? {
        Value::Object(obj) => {
            let mut result: HashMap<String, OrderBy> = HashMap::new();
            for (key, value) in obj {
                if let Value::String(v )= value{
                    result.insert(key.to_string(), get_order_by(&v));
                }
            };
            Some(result)
        }
        _ => None,
    }
}
fn init_data(request: &Request) -> Option<MaybeMany> {
    fn object_to_hashmap(obj: &Map<String, Value>) -> HashMap<String, Value> {
        obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
    match &request.data {
        Value::Object(obj) => {
            let result = object_to_hashmap(obj);
            Some(One(result))
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                return None;
            }
            if !arr.iter().all(|item| item.is_object()) {
                return None;
            }
            let items: Vec<HashMap<String, Value>> = arr
                .iter()
                .map(|item| {
                    object_to_hashmap(item.as_object().unwrap())
                })
                .collect();
            if items.len() == 1 {
                Some(One(items.into_iter().next().unwrap()))
            } else {
                Some(Many(items))
            }
        }
        _ => None,
    }
}
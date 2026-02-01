use crate::command_resolution::analytical_structure::parse_condition;
use crate::command_resolution::command_model::{AnalysisArgs, Condition};
use crate::model::{DataBase, MaybeMany, Request};
use serde_json::Value;
use std::collections::HashMap;
use serde_json::Error as JsonError;

pub(crate) struct CommandResolution {
    pub database: String,
    pub command: String,
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
            command: request.command,
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
fn init_order_by(request: &Request) -> Option<Vec<HashMap<String, String>>> {
    let order_by_value = request.args.get("order_by")?;
    let array = order_by_value.as_array()?;

    // 1. 先检查类型
    if !array.iter().all(|item| item.is_object()) {
        return None;
    }

    // 2. 尝试转换所有元素，这会得到一个 Result<Vec<T>, E>
    let result: Result<Vec<_>, _> = array
        .iter()
        .map(|item| convert_value_to_hashmap(item)) // 产生 Iterator<Item = Result<HashMap, _>>
        .collect();

    // 3. 将 Result 转换为 Option (Ok 变 Some, Err 变 None)
    result.ok()
}
fn convert_value_to_hashmap(value: &Value) -> Result<HashMap<String, String>, JsonError> {
    // 直接尝试将 Value 反序列化为 HashMap<String, String>
    serde_json::from_value(value.clone())
}
fn init_data(request: &Request) -> Option<MaybeMany> {
    let data_value = request.args.get("data")?;
    match data_value {
        Value::Object(obj) => {
            Some(MaybeMany::One(obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()))
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                return None;
            }
            let mut hashmaps = Vec::new();

            for item in vec {
                if let Value::Object(obj) = item {
                    // 将 Map 转换为 HashMap
                    let std_map: HashMap<String, Value> = obj
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();
                    hashmaps.push(std_map);
                } else {
                    // 如果数组中包含非对象元素，返回 None
                    return None;
                }
            }

            // 根据数量决定返回 One 还是 Many
            match hashmaps.len() {
                0 => None, // 理论上不会发生，因为上面检查过空值
                1 => Some(MaybeMany::One(hashmaps.into_iter().next().unwrap())),
                _ => Some(MaybeMany::Many(hashmaps)),
            }
        }
        _ => None
    }
}
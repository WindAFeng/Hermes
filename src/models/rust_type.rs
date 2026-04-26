use std::collections::HashMap;
use serde_json::Value;
fn from_value_to_rust(value: Value) -> RustType {
    match value {
        Value::String(s) => RustType::String(s),
        Value::Number(number) => {
            if let Some(i) = number.as_i64() {
                RustType::Integer(i)
            } else if let Some(u) = number.as_u64() {
                RustType::UnsignedInteger(u)
            } else if let Some(f) = number.as_f64() {
                RustType::Float(f)
            } else {
                RustType::None
            }
        }
        Value::Bool(b) => RustType::Bool(b),
        Value::Array(array) => {
            RustType::List(array.into_iter().map(from_value_to_rust).collect())
        }
        Value::Object(map) => {
            RustType::HashMap(
                map.into_iter()
                    .map(|(k, v)| (k, from_value_to_rust(v)))
                    .collect(),
            )
        }
        Value::Null => RustType::None,
    }
}
pub enum RustType {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Bool(bool),
    HashMap(HashMap<String, RustType>),
    List(Vec<RustType>),
    None
}
impl RustType {
    pub fn from_value(value: Value) -> RustType {
        from_value_to_rust(value)
    }
    pub fn to_string(&self) -> String {
        match self {
            RustType::String(s) => s.clone(),
            RustType::Integer(i) => i.to_string(),
            RustType::UnsignedInteger(i) => i.to_string(),
            RustType::Float(f) => f.to_string(),
            RustType::Bool(b) => b.to_string(),
            RustType::None => "None".to_string(),
            RustType::List(list) => {
                let items: Vec<String> = list.iter().map(|item| item.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            RustType::HashMap(map) => {
                let mut pairs: Vec<String> = Vec::new();
                for (k, v) in map {
                    // 键是字符串，直接用；值递归转字符串
                    pairs.push(format!("\"{}\": {}", k, v.to_string()));
                }
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}
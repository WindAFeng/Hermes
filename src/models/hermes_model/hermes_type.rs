use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type_visitor::HermesTypeVisitor;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Number, Value};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub enum HermesType {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    HashMap(HashMap<String, Self>),
    List(Vec<Self>),
    None,
}
impl PartialEq for HermesType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(a), Self::String(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::HashMap(a), Self::HashMap(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}
impl Serialize for HermesType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(s) => s.serialize(serializer),
            Self::Integer(i) => i.serialize(serializer),
            Self::Float(f) => {
                if f.is_finite() {
                    f.serialize(serializer)
                } else {
                    serializer.serialize_none()
                }
            }
            Self::Bool(b) => b.serialize(serializer),
            Self::HashMap(map) => {
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    map_ser.serialize_entry(k, v)?;
                }
                map_ser.end()
            }
            Self::List(vec) => {
                let mut seq_ser = serializer.serialize_seq(Some(vec.len()))?;
                for item in vec {
                    seq_ser.serialize_element(item)?;
                }
                seq_ser.end()
            }
            Self::None => serializer.serialize_none(),
        }
    }
}
impl<'de> Deserialize<'de> for HermesType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(HermesTypeVisitor)
    }
}
impl HermesType {
    pub fn from_serde_value(value: Value) -> Self {
        from_serde_value_(value)
    }
    pub fn to_serde_value(&self) -> Value {
        to_serde_value(self.clone())
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Integer(i) => i.to_string(),
            Self::Float(f) => f.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::None => "None".to_string(),
            Self::List(list) => {
                let items: Vec<String> = list.iter().map(|item| item.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Self::HashMap(map) => {
                let mut pairs: Vec<String> = Vec::new();
                for (k, v) in map {
                    pairs.push(format!("\"{}\": {}", k, v.to_string()));
                }
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
    pub fn from_redis_value(value: redis::Value) -> Self {
        from_redis_value_(value)
    }
    pub fn list_try_to_set(&self) -> Result<HashSet<String>, HermesError> {
        if let Self::List(vec) = self {
            let mut set = HashSet::with_capacity(vec.len());
            for item in vec {
                set.insert(item.to_string());
            }
            Ok(set)
        } else {
            Err(HermesError::Internal("Type Error".to_string()))
        }
    }
}
impl HermesType {
    pub fn as_string(&self) -> Result<&String, HermesError> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_integer(&self) -> Result<i64, HermesError> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_float(&self) -> Result<f64, HermesError> {
        match self {
            Self::Float(f) => Ok(*f),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_bool(&self) -> Result<bool, HermesError> {
        match self {
            Self::Bool(b) => Ok(*b),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_hash(&self) -> Result<&HashMap<String, Self>, HermesError> {
        match self {
            Self::HashMap(hm) => Ok(hm),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_list(&self) -> Result<&Vec<Self>, HermesError> {
        match self {
            Self::List(vec) => Ok(vec),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn as_none(&self) -> Result<&Self, HermesError> {
        match self {
            Self::None => Ok(self),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
}
impl HermesType {
    pub fn redis_string(&self) -> Result<String, HermesError> {
        match &self {
            Self::String(s) => Ok(s.clone()),
            Self::Integer(i) => Ok(i.to_string()),
            Self::Float(f) => Ok(f.to_string()),
            Self::Bool(b) => Ok(b.to_string()),
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn redis_hash(&self) -> Result<HashMap<String, String>, HermesError> {
        match &self {
            Self::HashMap(map) => {
                let mut hashmap: HashMap<String, String> = HashMap::with_capacity(map.len());
                for (k, v) in map.iter() {
                    let value = v.redis_string()?;
                    hashmap.insert(k.to_string(), value);
                }
                Ok(hashmap)
            }
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn redis_list(&self) -> Result<Vec<String>, HermesError> {
        match &self {
            Self::List(array) => {
                let mut vec = Vec::with_capacity(array.len());
                for i in array {
                    let item = i.redis_string()?;
                    vec.push(item);
                }
                Ok(vec)
            }
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn redis_set(&self) -> Result<Vec<String>, HermesError> {
        match &self {
            Self::List(array) => {
                let mut set = HashSet::with_capacity(array.len());
                for i in array {
                    set.insert(i.redis_string()?);
                }
                let mut list = Vec::with_capacity(set.len());
                for i in set {
                    list.push(i);
                }
                Ok(list)
            }
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
    pub fn redis_zset(&self) -> Result<HashMap<String, i64>, HermesError> {
        match &self {
            Self::HashMap(map) => {
                let mut hashmap: HashMap<String, i64> = HashMap::with_capacity(map.len());
                for (k, v) in map.iter() {
                    let value = v.as_integer().unwrap();
                    hashmap.insert(k.to_string(), value);
                }
                Ok(hashmap)
            }
            _ => Err(HermesError::Internal("Type Error".to_string())),
        }
    }
}
fn from_serde_value_(value: Value) -> HermesType {
    match value {
        Value::String(s) => HermesType::String(s),
        Value::Number(number) => {
            if let Some(i) = number.as_i64() {
                HermesType::Integer(i)
            } else if let Some(f) = number.as_f64() {
                HermesType::Float(f)
            } else {
                HermesType::None
            }
        }
        Value::Bool(b) => HermesType::Bool(b),
        Value::Array(array) => HermesType::List(array.into_iter().map(from_serde_value_).collect()),
        Value::Object(map) => HermesType::HashMap(
            map.into_iter()
                .map(|(k, v)| (k, from_serde_value_(v)))
                .collect(),
        ),
        Value::Null => HermesType::None,
    }
}
fn from_redis_value_(value: redis::Value) -> HermesType {
    match value {
        redis::Value::Double(f) => HermesType::Float(f),
        redis::Value::Int(i) => HermesType::Integer(i),
        redis::Value::SimpleString(s) => HermesType::String(s),
        redis::Value::Boolean(b) => HermesType::Bool(b),
        redis::Value::Set(set) => {
            let mut ls = Vec::with_capacity(set.len());
            for item in set {
                ls.push(from_redis_value_(item));
            }
            HermesType::List(ls)
        }
        redis::Value::Array(array) => {
            let mut ls = Vec::with_capacity(array.len());
            for item in array {
                ls.push(from_redis_value_(item));
            }
            HermesType::List(ls)
        }
        redis::Value::Map(map) => {
            let mut hashmap: HashMap<String, HermesType> = HashMap::with_capacity(map.len());
            for (k, v) in map {
                if let HermesType::String(s) = from_redis_value_(k) {
                    hashmap.insert(s, from_redis_value_(v));
                }
            }
            HermesType::HashMap(hashmap)
        }
        _ => HermesType::None,
    }
}
fn to_serde_value(value: HermesType) -> Value {
    match value {
        HermesType::String(s) => Value::String(s),
        HermesType::Integer(i) => Value::Number(i.into()),
        HermesType::Float(f) => Value::Number(Number::from_f64(f).unwrap()),
        HermesType::Bool(b) => Value::Bool(b),
        HermesType::HashMap(hashmap) => {
            let mut object: Map<String, Value> = Map::new();
            for (k, v) in hashmap {
                object.insert(k, to_serde_value(v));
            }
            Value::Object(object)
        }
        HermesType::List(list) => {
            let mut array: Vec<Value> = Vec::with_capacity(list.len());
            for item in list {
                array.push(to_serde_value(item));
            }
            Value::Array(array)
        }
        HermesType::None => Value::Null,
    }
}

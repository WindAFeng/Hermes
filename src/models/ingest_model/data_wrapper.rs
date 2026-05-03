use serde::{Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use serde::ser::{SerializeMap, SerializeSeq};

#[derive(Debug, Clone)]
pub enum DataWrapper {
    One(HashMap<String, Value>),
    Many(Vec<HashMap<String, Value>>),
}
impl Serialize for DataWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match &self {
            DataWrapper::One(x) => {
                let mut map = serializer.serialize_map(Some(x.len()))?;
                for (k, v) in x {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            },
            DataWrapper::Many(x) => {
                let mut seq = serializer.serialize_seq(Some(x.len()))?;
                for item in x {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
        }
    }
}
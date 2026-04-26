use serde::{Serialize, Serializer};

#[derive(Debug)]
pub enum ResponseMessageType {
    Success,
    Error(String),
}

impl Serialize for ResponseMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_str("Success"),
            Self::Error(e) => serializer.serialize_str(&format!("HermesError: {}", e)),
        }
    }
}


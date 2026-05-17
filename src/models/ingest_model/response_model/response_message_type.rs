use serde::{Serialize, Serializer};
use crate::models::hermes_model::hermes_error::HermesError;

#[derive(Debug)]
pub enum ResponseMessageType {
    Success,
    Error(HermesError),
}

impl Serialize for ResponseMessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_str("Success"),
            Self::Error(e) => serializer.serialize_str(&format!("Hermes{}", e)),
        }
    }
}


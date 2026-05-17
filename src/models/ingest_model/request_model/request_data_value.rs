use serde::Deserialize;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::hermes_model::hermes_type::HermesType;

#[derive(Deserialize, Debug, Clone)]
pub struct RequestDataValue {
    #[serde(rename = "type")]
    type_: String,
    pub data: HermesType
}
impl RequestDataValue {
    pub fn type_to_json(&self) -> Result<String, HermesError> {
        serde_json::to_string(self.type_.as_str()).map_err(HermesError::from)
    }
}
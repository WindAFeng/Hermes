use serde::Deserialize;
use crate::errors::HermesError;
use crate::models::hermes_types::HermesTypes;
#[derive(Deserialize, Debug)]
pub struct RedisArgs{
    pub value_type: Option<HermesTypes>,
    pub keys: Option<Vec<String>>,
}
impl RedisArgs{
    fn return_error(&self, lack_arg: &str) -> HermesError{
        HermesError::Internal(format!("Not Found arg '{}'", lack_arg))
    }
    pub fn get_value_type(&self)->Result<HermesTypes, HermesError> {
        match &self.value_type {
            Some(value_type) => Ok(value_type.clone()),
            None => Err(
                self.return_error("value_type")
            )
        }
    }
    pub fn get_keys(&self)->Result<Vec<String>, HermesError> {
        match &self.keys { 
            Some(keys) => Ok(keys.clone()),
            None => Err(
                self.return_error("keys")
            )
        }
    }
}
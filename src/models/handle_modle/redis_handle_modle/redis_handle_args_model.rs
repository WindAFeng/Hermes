use serde::Deserialize;
use crate::models::hermes_types::HermesTypes;
#[derive(Deserialize, Debug)]
pub struct RedisArgs{
    pub value_type: Option<HermesTypes>,
}
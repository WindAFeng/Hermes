use serde::Deserialize;
use crate::models::hermes_types::HermesTypes;
#[derive(Deserialize, Debug)]
pub struct RedisArgs{
    #[serde(rename = "type")]
    pub type_: Option<HermesTypes>,
}
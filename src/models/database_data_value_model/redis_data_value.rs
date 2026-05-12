use crate::models::database_data_type_model::redis_data_type::RedisDataType;
use crate::models::hermes_model::hermes_type::HermesType;

#[derive(Clone)]
pub struct RedisDataValue {
    pub type_: RedisDataType,
    pub data: HermesType,
}
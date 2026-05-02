use serde::Deserialize;
use crate::models::handle_modle::redis_handle_modle::redis_command_type::RedisCommandType;
use crate::models::hermes_types::HermesTypes;
#[derive(Deserialize, Debug)]
pub struct RedisArgs{
    pub value_type: Option<HermesTypes>,
    pub command_type: Option<RedisCommandType>,
}
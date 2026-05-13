use crate::models::hermes_model::hermes_type::HermesType;

#[derive(Clone)]
pub struct CacheValue {
    pub type_: String,
    pub value: HermesType,
}
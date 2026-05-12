use crate::models::hermes_model::hermes_error::HermesError;
use async_trait::async_trait;
use crate::models::hermes_model::hermes_type::HermesType;
#[async_trait]
pub trait DatabaseAdapt : Send + Sync {
    async fn add(&self) -> Result<(), HermesError>;
    async fn get(&self) -> Result<Option<HermesType>, HermesError>;
    async fn delete(&self) -> Result<(), HermesError>;
    async fn update(&self) -> Result<(), HermesError>;
    async fn use_(&self) -> Result<(), HermesError>;
}
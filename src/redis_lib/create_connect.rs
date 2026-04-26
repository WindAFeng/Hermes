use redis::aio::MultiplexedConnection;
use redis::{Client};
use crate::errors::HermesError;

pub async fn create_connect() -> Result<MultiplexedConnection, HermesError> {
    let client = Client::open("redis://127.0.0.1:6379")?;
    let con = client.get_multiplexed_async_connection().await?;
    Ok(con)
}
use redis::aio::MultiplexedConnection;
use redis::{Client};
use crate::errors::HermesError;

pub async fn establish_redis_connection(host: &str, port: &str) -> Result<MultiplexedConnection, HermesError> {
    let addr = format!("redis://{}:{}", host, port);
    let client = match Client::open(addr) {
        Ok(c) => c,
        Err(e) => return Err(HermesError::from(e))
    };
    let con = client.get_multiplexed_async_connection().await?;
    Ok(con)
}
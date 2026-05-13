use crate::command_executor::CommandRouter;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::ingest_model::response_model::response::Response;
use crate::utils::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn read_data(socket: &mut TcpStream, buf: &mut [u8]) -> Result<Option<usize>, HermesError> {
    match socket.read(buf).await {
        Ok(0) => {
            log::debug("Client disconnected");
            Ok(None)
        }
        Ok(n) => Ok(Some(n)),
        Err(e) => {
            log::warn(format!("TCP Read Error: {}", e));
            Err(HermesError::Network(e.to_string()))
        }
    }
}

pub async fn socket_handel(mut socket: TcpStream) -> Result<(), HermesError> {
    let mut buf = [0; 4096];
    let data = read_data(&mut socket, &mut buf).await;
    while let Ok(Some(n)) = data {
        let req = serde_json::from_slice(&buf[..n])?;
        let cmd_rst = CommandRouter::new(req).get_result().await;
        let resp_bytes = Response::new(cmd_rst).to_bytes()?;
        socket.write_all(&resp_bytes).await?;
    }
    if let Err(e) = data{
        return Err(HermesError::Network(e.to_string()))
    }
    Ok(())
}

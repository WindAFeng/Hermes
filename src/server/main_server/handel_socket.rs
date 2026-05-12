use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::ingest_model::request_model::request::Request;
use crate::utils::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::command_executor::CommandExecutor;
use crate::models::ingest_model::response_model::response::Response;

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
fn parse_request(buf: &[u8]) -> Result<Request, HermesError> {
    serde_json::from_slice(buf).map_err(HermesError::from)
}

pub async fn socket_handel(mut socket: TcpStream) -> Result<(), HermesError> {
    let mut buf = [0; 4096];
    loop {
        let result = match read_data(&mut socket, &mut buf).await {
            Ok(Some(n)) => {
                let req = parse_request(&buf[..n])?;
                let cmd_rst = CommandExecutor::new(req).get_result().await;
                let resp_bytes = Response::new(cmd_rst).to_bytes()?;
                socket.write_all(&resp_bytes).await?;
                Ok(())
            },
            Ok(None) => break,
            Err(e) => Err(HermesError::from(e))
        };
        result?
    }
    Ok(())
}

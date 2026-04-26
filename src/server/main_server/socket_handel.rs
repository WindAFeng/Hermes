use crate::errors::HermesError;
use crate::models::ingest_model::request::Request;
use crate::utils::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::command_handle::handle::CommandHandle;

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
async fn get_result(buf: &[u8]) -> Result<Request, HermesError> {
    serde_json::from_slice(buf).map_err(|e| HermesError::from(e))
}

pub async fn socket_handel(mut socket: TcpStream) -> Result<(), HermesError> {
    let mut buf = [0; 4096];
    loop {
        let n = match read_data(&mut socket, &mut buf).await {
            Ok(Some(n)) => n,
            _ => break,
        };
        let req = match get_result(&buf[..n]).await {
            Ok(r) => r,
            Err(e) => {
                log::error(format!("JSON解析失败: {}", e));
                continue;
            }
        };
        let handle = CommandHandle::new(req);
        let resp = match handle.get().await {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let resp_bytes = serde_json::to_vec(&resp)?;
        if let Err(e) = socket.write_all(&resp_bytes).await {
            log::error(format!("TCP Write Error: {}", e));
        }
    }
    Ok(())
}

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::model::{Request, Response};
use crate::command_resolution::command_resolution::CommandResolution;

pub async fn handle_client(mut socket: TcpStream) {
    // 创建4kb缓冲
    let mut buf = [0u8; 4096];

    if let Err(e) = async {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "空数据",
            ));
        }

        let req: Request = serde_json::from_slice(&buf[..n])
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let command_resolution = CommandResolution::new(req);
        println!("{:?}", command_resolution.args);
        let resp = Response {
            code: 0,
            message: "success".to_string(),
            data: None,
        };

        let resp_bytes = serde_json::to_vec(&resp)?;
        socket.write_all(&resp_bytes).await?;
        Ok::<(), std::io::Error>(())
    }
        .await
    {
        eprintln!("处理客户端时出错: {}", e);
    }
}
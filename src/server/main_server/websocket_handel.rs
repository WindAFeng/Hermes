use futures_util::stream::SplitSink;
use crate::errors::HermesError;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, accept_async};
use tokio_tungstenite::tungstenite::{Message, Error};
use tokio_tungstenite::tungstenite::protocol::frame::Utf8Bytes;
use crate::utils::log;
use crate::models::ingest_model::{request::Request, response::Response};
use crate::models::ingest_model::response_message_type::ResponseMessageType;
use crate::models::ingest_model::response_code_type::ResponseCodeType;

async fn get_ws_stream(stream: TcpStream) -> Result<WebSocketStream<TcpStream>, HermesError> {
    match accept_async(stream).await {
        Ok(s) => {
            log::debug("WebSocket Handshake successful");
            Ok(s)
        }
        Err(e) => {
            log::error(format!("WebSocket Handshake failed: {}", e));
            Err(HermesError::from(e))
        }
    }
}
fn get_message(message_result: Result<Message, Error>) ->Result<Option<Message>, HermesError> {
    match message_result {
        Ok(message) => {
            if message.is_close(){
                return Ok(None);
            }
            Ok(Some(message))
        }
        Err(e) => {
            log::error(format!("WebSocket Handshake failed: {}", e));
            Err(HermesError::from(e))
        }
    }
}
fn to_str(message: &Message) -> Result<&str, HermesError> {
    match message.to_text() {
        Ok(text) => Ok(text),
        Err(e) => Err(HermesError::from(e))
    }
}
fn get_request(text: &str) -> Result<Request, HermesError> {
    match serde_json::from_str::<Request>(text) {
        Ok(request) => {
            Ok(request)
        }
        Err(e) => {
            log::error(format!("Failed to deserialize request: {}", e));
            Err(HermesError::from(e))
        }
    }
}
fn get_json_str(resp: &Response) -> Result<Option<Utf8Bytes>, HermesError> {
    match serde_json::to_string(resp) {
        Ok(json) => Ok(Some(Utf8Bytes::from(json))),
        Err(e) => Err(HermesError::from(e))
    }
}

async fn send_json(sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>, json_str: Utf8Bytes) -> Result<(), HermesError> {
    match sender.send(Message::Text(json_str)).await {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error(format!("Failed to send message: {}", e));
            Err(HermesError::from(e))
        }
    }
}
pub async fn websocket_handel(stream: TcpStream) -> Result<(), HermesError> {
    let ws_stream = get_ws_stream(stream).await?;
    let (mut sender, mut receiver) = ws_stream.split();
    loop {
        if let Some(message_result) = receiver.next().await {
            let message = match get_message(message_result) {
                Ok(Some(message)) => message,
                _ => break
            };
            if message.is_text() {
                let text = match to_str(&message) {
                    Ok(text) => text,
                    Err(_) => break
                };
                let request = match get_request(text) {
                    Ok(request) => request,
                    Err(_) => break
                };
                let resp = Response {
                    code: ResponseCodeType::Success,
                    message: ResponseMessageType::Success,
                    data: None,
                };
                let json_str = match get_json_str(&resp) {
                    Ok(Some(json_str)) => json_str,
                    _ => break
                };
                if let Err(_) = send_json(&mut sender, json_str).await {
                    break;
                }
            }else if message.is_close() {
                break;
            }
        }
    }
    Ok(())
}

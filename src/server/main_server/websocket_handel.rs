use futures_util::stream::{SplitSink, SplitStream};
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
fn to_json(resp: &Response) -> Result<Option<Utf8Bytes>, HermesError> {
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
async fn websocket_connect_handle(receiver: &mut SplitStream<WebSocketStream<TcpStream>>, sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>) -> Result<(), HermesError> {
    if let Some(message_result) = receiver.next().await {
        let message = match get_message(message_result) {
            Ok(Some(message)) => message,
            _ => return Err(HermesError::Network("Can't get message".to_string()))
        };
        if message.is_text() {
            check_message_is_text(&message, sender).await
        }else if message.is_close() {
            return Err(HermesError::Internal("Can't close websocket".to_string()));
        }
        else {
            return Err(HermesError::Internal("Can't send message to websocket".to_string()));
        }
    }
    else {
        Err(HermesError::Internal("Can't get message from websocket".to_string()))
    }
}
async fn check_message_is_text(message: &Message, sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>)  -> Result<(), HermesError> {
    let text = match to_str(message) {
        Ok(text) => text,
        Err(error) => return Err(HermesError::from(error))
    };
    let request = match get_request(text) {
        Ok(request) => request,
        Err(error) => return Err(HermesError::from(error))
    };
    let bytes = match send_response(&request).await {
        Ok(bytes) => bytes,
        Err(error) => return Err(HermesError::from(error))
    };
    if let Err(error) = send_json(sender, bytes).await {
        return Err(HermesError::from(error));
    }
    Ok(())
}
async fn send_response(req: &Request) -> Result<Utf8Bytes, HermesError> {
    let resp = Response {
        code: ResponseCodeType::Success,
        message: ResponseMessageType::Success,
        data: None,
    };
    match to_json(&resp) {
        Ok(Some(json_str)) => Ok(json_str),
        _ => return Err(HermesError::Internal("Can't get response json".to_string()))
    }
}
pub async fn websocket_handel(stream: TcpStream) -> Result<(), HermesError> {
    let ws_stream = get_ws_stream(stream).await?;
    let (mut sender, mut receiver) = ws_stream.split();
    loop {
        return match websocket_connect_handle(&mut receiver, &mut sender).await {
            Ok(_) => Ok(()),
            Err(_) => Err(HermesError::Network("Error while connecting to websocket".to_string()))
        };
    }
}

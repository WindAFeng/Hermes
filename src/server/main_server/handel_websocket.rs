use crate::command_executor::CommandExecutor;
use crate::models::hermes_model::hermes_error::HermesError;
use crate::models::ingest_model::request_model::request::Request;
use crate::models::ingest_model::response_model::response::Response;
use crate::utils::log;
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::frame::Utf8Bytes;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::{WebSocketStream, accept_async};

fn get_message(message_result: Result<Message, Error>) -> Result<Option<Message>, HermesError> {
    match message_result {
        Ok(message) => {
            if message.is_close() {
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
async fn websocket_connect_handle(
    message_result: Result<Message, Error>,
    sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
) -> Result<(), HermesError> {
    let message = match get_message(message_result) {
        Ok(Some(message)) => message,
        _ => return Err(HermesError::Network("Can't get message".to_string())),
    };
    if message.is_text() {
        check_message_is_text(&message, sender).await
    } else if message.is_close() {
        return Err(HermesError::Internal("Can't close websocket".to_string()));
    } else {
        return Err(HermesError::Internal(
            "Can't send message to websocket".to_string(),
        ));
    }
}
async fn check_message_is_text(
    message: &Message,
    sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
) -> Result<(), HermesError> {
    let text = message.to_text().map_err(HermesError::from)?;
    let request = serde_json::from_str::<Request>(text)?;
    let cmd_rst = CommandExecutor::new(request).get_result().await;
    let string = serde_json::to_string(&Response::new(cmd_rst))?;
    sender
        .send(Message::Text(Utf8Bytes::from(string)))
        .await
        .map_err(HermesError::from)
}
pub async fn websocket_handle(stream: TcpStream) -> Result<(), HermesError> {
    let ws_stream = accept_async(stream).await?;
    let (mut sender, mut receiver) = ws_stream.split();
    while let Some(result) = receiver.next().await {
        if let Err(e) = websocket_connect_handle(result, &mut sender).await {
            return Err(e);
        }
    }
    Ok(())
}

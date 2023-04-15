use axum::extract::ws::{CloseFrame, Message};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// チャットのメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessage {
    Init(Vec<String>),
    Text(String),
}

/// axumのmessageのwrapper
/// Initの状態を扱えるようにする
#[derive(Debug, Clone)]
pub enum SocketMessage {
    Message(ChatMessage),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<CloseFrame<'static>>),
}

impl From<Message> for SocketMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => SocketMessage::Message(ChatMessage::Text(text)),
            Message::Binary(bin) => SocketMessage::Binary(bin),
            Message::Ping(ping) => SocketMessage::Ping(ping),
            Message::Pong(pong) => SocketMessage::Pong(pong),
            Message::Close(close) => SocketMessage::Close(close),
        }
    }
}

impl From<SocketMessage> for Message {
    fn from(msg: SocketMessage) -> Self {
        match msg {
            SocketMessage::Message(msg) => Message::Text(msg.into()),
            SocketMessage::Binary(bin) => Message::Binary(bin),
            SocketMessage::Ping(ping) => Message::Ping(ping),
            SocketMessage::Pong(pong) => Message::Pong(pong),
            SocketMessage::Close(close) => Message::Close(close),
        }
    }
}

impl TryFrom<SocketMessage> for ChatMessage {
    type Error = ();

    fn try_from(msg: SocketMessage) -> Result<Self, Self::Error> {
        match msg {
            SocketMessage::Message(msg) => Ok(msg),
            _ => Err(()),
        }
    }
}

impl From<ChatMessage> for SocketMessage {
    fn from(msg: ChatMessage) -> Self {
        SocketMessage::Message(msg)
    }
}

impl From<ChatMessage> for String {
    fn from(msg: ChatMessage) -> Self {
        serde_json::to_string(&msg).unwrap()
    }
}

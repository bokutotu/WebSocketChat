use axum::extract::ws::{CloseFrame, Message};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use std::convert::TryFrom;

use crate::db::{MessageRoom, Comment};

/// メッセージの送受信の際に使用する構造体
/// メッセージルームのid(usize)
/// メッセージの送信主(string)
/// メッセージの内容(string)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMessage {
    pub room_id: Uuid,
    pub comment: Comment,
}

/// チャットのメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatMessageType {
    Init(Vec<Comment>),
    RoomMessage(RoomMessage),
    Rooms(Vec<String>),
}

impl ChatMessageType {
    pub fn into_room_message(self) -> Option<RoomMessage> {
        match self {
            ChatMessageType::RoomMessage(msg) => Some(msg),
            _ => None,
        }
    }
}

/// axumのmessageのwrapper
/// Initの状態を扱えるようにする
#[derive(Debug, Clone)]
pub enum WebSocketMessageType {
    Message(ChatMessageType),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<CloseFrame<'static>>),
}

impl From<Message> for WebSocketMessageType {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => { 
                let msg = serde_json::from_str(&text).unwrap();
                WebSocketMessageType::Message(msg)
            },
            Message::Binary(bin) => WebSocketMessageType::Binary(bin),
            Message::Ping(ping) => WebSocketMessageType::Ping(ping),
            Message::Pong(pong) => WebSocketMessageType::Pong(pong),
            Message::Close(close) => WebSocketMessageType::Close(close),
        }
    }
}

impl From<WebSocketMessageType> for Message {
    fn from(msg: WebSocketMessageType) -> Self {
        match msg {
            WebSocketMessageType::Message(msg) => Message::Text(msg.into()),
            WebSocketMessageType::Binary(bin) => Message::Binary(bin),
            WebSocketMessageType::Ping(ping) => Message::Ping(ping),
            WebSocketMessageType::Pong(pong) => Message::Pong(pong),
            WebSocketMessageType::Close(close) => Message::Close(close),
        }
    }
}

impl TryFrom<WebSocketMessageType> for ChatMessageType {
    type Error = ();

    fn try_from(msg: WebSocketMessageType) -> Result<Self, Self::Error> {
        match msg {
            WebSocketMessageType::Message(msg) => Ok(msg),
            _ => Err(()),
        }
    }
}

impl From<ChatMessageType> for WebSocketMessageType {
    fn from(msg: ChatMessageType) -> Self {
        WebSocketMessageType::Message(msg)
    }
}

impl From<ChatMessageType> for String {
    fn from(msg: ChatMessageType) -> Self {
        serde_json::to_string(&msg).unwrap()
    }
}

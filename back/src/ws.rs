use axum::extract::ws::{Message, WebSocket};

use futures::sink::SinkExt;
use futures::stream::{SplitSink, SplitStream, StreamExt};

use crate::message::SocketMessage;

/// websocketのwrapper
/// axumのMessageを使わないようにする
pub struct ChatWebSocket {
    sink: ChatSink,
    stream: ChatStream,
}

impl ChatWebSocket {
    pub fn new(socket: WebSocket) -> Self {
        let (sink, socket) = socket.split();
        Self {
            stream: ChatStream::new(socket),
            sink: ChatSink::new(sink),
        }
    }

    // async fn next(&mut self) -> Option<Result<SocketMessage, axum::Error>> {
    //     self.stream.next().await.map(|msg| msg.map(Into::into))
    // }

    pub async fn send(&mut self, msg: SocketMessage) -> Result<(), axum::Error> {
        self.sink.send(msg).await.map_err(Into::into)
    }

    pub fn split(self) -> (ChatSink, ChatStream) {
        (self.sink, self.stream)
    }
}

pub struct ChatStream(SplitStream<WebSocket>);
pub struct ChatSink(SplitSink<WebSocket, Message>);

impl ChatStream {
    pub fn new(socket: SplitStream<WebSocket>) -> Self {
        Self(socket)
    }

    pub async fn next(&mut self) -> Option<Result<SocketMessage, axum::Error>> {
        self.0.next().await.map(|msg| msg.map(Into::into))
    }
}

impl ChatSink {
    pub fn new(sink: SplitSink<WebSocket, Message>) -> Self {
        Self(sink)
    }

    pub async fn send(&mut self, msg: SocketMessage) -> Result<(), axum::Error> {
        self.0.send(msg.into()).await.map_err(Into::into)
    }
}

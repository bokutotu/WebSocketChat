use axum::extract::connect_info::ConnectInfo;
use axum::extract::State;
use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

use serde::{Deserialize, Serialize};

use tokio::sync::broadcast;

use std::net::SocketAddr;
// use std::ops::ControlFlow;
use std::sync::{Arc, Mutex};
// use std::ops::{Deref, DerefMut};

use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(32);
    let db = Arc::new(Mutex::new(MessageDb::new(tx)));
    let app = Router::new().route("/ws", get(ws_handler)).with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 1919));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<Arc<Mutex<MessageDb>>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(ChatWebSocket::new(socket), addr, state))
}

async fn handle_socket(
    mut stream: ChatWebSocket,
    who: SocketAddr,
    state: State<Arc<Mutex<MessageDb>>>,
) {
    // 誰が接続したかを表示する
    println!("`{who}` connected.");
    println!(
        "initial state is {:#?}",
        state.lock().unwrap().init_message()
    );

    let tx = state.lock().unwrap().tx.clone();

    // 履歴を送信する
    let v = state.lock().unwrap().init_message();
    stream
        .send(SocketMessage::Message(ChatMessage::Init(v)))
        .await
        .unwrap();

    let (mut sender_ws, mut reciver_ws) = stream.split();

    // 1. WebSocketの送信を受信
    // 2. 受信したメッセージをbroadcastに送信
    // 3. stateを更新
    let s = state.clone();
    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = reciver_ws.next().await {
            let msg = msg;
            println!("`{who}` sent {msg:?}");
            tx.send(msg.clone()).unwrap();
            {
                let mut state = s.lock().unwrap();
                state.add_message(msg);
            }
        }
    });

    // 1. broadcastからメッセージを受信
    // 2. WebSocketに送信
    let mut rx = { state.lock().unwrap().tx.broadcast() };
    let mut recv_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("`{who}` received {msg:?}");
            sender_ws.send(msg).await.unwrap();
        }
    });

    // 両方のタスクが終了するまで待つ
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

/// websocketのwrapper
/// axumのMessageを使わないようにする
struct ChatWebSocket {
    sink: ChatSink,
    stream: ChatStream,
}

impl ChatWebSocket {
    fn new(socket: WebSocket) -> Self {
        let (sink, socket) = socket.split();
        Self {
            stream: ChatStream::new(socket),
            sink: ChatSink::new(sink),
        }
    }

    // async fn next(&mut self) -> Option<Result<SocketMessage, axum::Error>> {
    //     self.stream.next().await.map(|msg| msg.map(Into::into))
    // }

    async fn send(&mut self, msg: SocketMessage) -> Result<(), axum::Error> {
        self.sink.send(msg).await.map_err(Into::into)
    }

    fn split(self) -> (ChatSink, ChatStream) {
        (self.sink, self.stream)
    }
}

struct ChatStream(SplitStream<WebSocket>);
struct ChatSink(SplitSink<WebSocket, Message>);

impl ChatStream {
    fn new(socket: SplitStream<WebSocket>) -> Self {
        Self(socket)
    }

    async fn next(&mut self) -> Option<Result<SocketMessage, axum::Error>> {
        self.0.next().await.map(|msg| msg.map(Into::into))
    }
}

impl ChatSink {
    fn new(sink: SplitSink<WebSocket, Message>) -> Self {
        Self(sink)
    }

    async fn send(&mut self, msg: SocketMessage) -> Result<(), axum::Error> {
        self.0.send(msg.into()).await.map_err(Into::into)
    }
}

/// チャットのメッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ChatMessage {
    Init(Vec<String>),
    Text(String),
}

/// axumのmessageのwrapper
/// Initの状態を扱えるようにする
#[derive(Debug, Clone)]
enum SocketMessage {
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

#[derive(Debug, Clone)]
struct MessageTx(broadcast::Sender<SocketMessage>);
struct MessageRx(broadcast::Receiver<SocketMessage>);

impl MessageTx {
    fn new(tx: broadcast::Sender<SocketMessage>) -> Self {
        Self(tx)
    }

    fn broadcast(&self) -> MessageRx {
        MessageRx(self.0.subscribe())
    }

    fn send<I: Into<SocketMessage>>(
        &self,
        msg: I,
    ) -> Result<usize, broadcast::error::SendError<SocketMessage>> {
        let msg = msg.into();
        self.0.send(msg)
    }
}

impl MessageRx {
    async fn recv(&mut self) -> Result<SocketMessage, broadcast::error::RecvError> {
        self.0.recv().await
    }
}

struct MessageDb {
    messages: Vec<String>,
    tx: MessageTx,
}

impl MessageDb {
    fn new(tx: broadcast::Sender<SocketMessage>) -> Self {
        Self {
            messages: Vec::new(),
            tx: MessageTx::new(tx),
        }
    }

    fn add_message<T>(&mut self, msg: T)
    where
        T: TryInto<ChatMessage> + Clone + Into<SocketMessage>,
        <T as TryInto<ChatMessage>>::Error: std::fmt::Debug,
    {
        println!("this is add_message");
        let msg_chat: ChatMessage = msg.try_into().unwrap();
        let msg_str: String = msg_chat.into();
        self.messages.push(msg_str);
    }

    fn init_message(&self) -> Vec<String> {
        self.messages.clone()
    }
}

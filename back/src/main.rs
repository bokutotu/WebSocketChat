use axum::extract::connect_info::ConnectInfo;
use axum::extract::State;
use axum::{
    extract::{ws::WebSocketUpgrade, TypedHeader},
    response::IntoResponse,
    routing::get,
    Router,
};

use tokio::sync::broadcast;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub mod channel;
pub mod db;
pub mod message;
pub mod ws;

use db::MessageDb;
use message::{ChatMessage, SocketMessage};
use ws::ChatWebSocket;

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

    let tx = state.lock().unwrap().tx().clone();

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
    let mut rx = { state.lock().unwrap().tx().broadcast() };
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

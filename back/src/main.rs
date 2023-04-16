use axum::extract::connect_info::ConnectInfo;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::{
    http::{HeaderValue, Method},
    extract::{ws::WebSocketUpgrade, TypedHeader},
    response::IntoResponse,
    routing::get,
    Router,
    Json,
};

use tower_http::cors::CorsLayer;

use uuid::Uuid;

use serde::Deserialize;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use http::header;

pub mod channel;
pub mod db;
pub mod message;
pub mod ws;

use db::{MessageDb, Members};
use message::{ChatMessageType, WebSocketMessageType};
use ws::ChatWebSocket;
use channel::MessageTx;

struct AppState {
    chat_rooms: Arc<Mutex<MessageDb>>,
    members: Arc<Mutex<Members>>,
    tx: Arc<Mutex<MessageTx>>,
}

impl AppState {
    pub fn new() -> Self {
        let chat_rooms = Arc::new(Mutex::new(MessageDb::new()));
        let members = Arc::new(Mutex::new(Members::new()));
        let (tx, _) = channel::channel();
        let tx = Arc::new(Mutex::new(tx));
        Self {
            chat_rooms,
            members,
            tx,
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();
    let app_state = Arc::new(state);
    let app = Router::new()
        .route(
            "/ws:room_id", 
            get(ws_handler)
        )
        .route(
            "/members", 
            get(get_all_members)
            .post(add_member)
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET, 
                    Method::POST, 
                    Method::PUT, 
                    Method::DELETE
                ])
                .allow_headers([header::CONTENT_TYPE])
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 1919));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn get_all_members(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<Arc<AppState>>
) -> impl IntoResponse {
    println!("{addr} is requesting all members.");
    let members = state.members.lock().unwrap();
    let members = members.to_vec();
    println!("{addr} is requesting all members.");
    println!("result: {:#?}", members);
    Json(members)
}

async fn add_member(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<Arc<AppState>>,
    member: String,
) -> impl IntoResponse {
    let mut members = state.members.lock().unwrap();
    let m = members.add_member(&member);
    println!("{addr} is adding a member.");
    println!("result: {:#?}", m);
    (StatusCode::CREATED, Json(m))
}

async fn ws_handler(
    Path(room_id): Path<Uuid>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| 
        handle_socket(
            ChatWebSocket::new(socket), 
            addr, 
            room_id, 
            state.tx.clone(), 
            state.chat_rooms.clone()
        )
    )
}

async fn handle_socket(
    mut stream: ChatWebSocket,
    who: SocketAddr,
    room_id: Uuid,
    tx: Arc<Mutex<MessageTx>>,
    db: Arc<Mutex<MessageDb>>,
) {
    // 誰が接続したかを表示する
    println!("`{who}` connected.");
    println!(
        "initial state is {:#?}",
        db.lock().unwrap().get_message_history(room_id)
    );

    let tx = tx.lock().unwrap().clone();
    let tx_clone = tx.clone();

    // 履歴を送信する
    let v = db.lock().unwrap().get_message_history(room_id);
    stream
        .send(WebSocketMessageType::Message(v))
        .await
        .unwrap();

    let (mut sender_ws, mut reciver_ws) = stream.split();

    // 1. WebSocketの送信を受信
    // 2. 受信したメッセージをbroadcastに送信
    // 3. stateを更新
    let s = db.clone();
    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = reciver_ws.next().await {
            println!("`{who}` sent {msg:?}");
            let msg: ChatMessageType = msg.try_into().unwrap();
            tx.send(msg.clone()).unwrap();
            {
                let mut state = s.lock().unwrap();
                state.add_message(msg);
            }
        }
    });

    // 1. broadcastからメッセージを受信
    // 2. WebSocketに送信
    let mut rx = { tx_clone.subscribe() };
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

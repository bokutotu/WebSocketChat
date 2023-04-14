use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::extract::connect_info::ConnectInfo;

use std::ops::ControlFlow;
use std::net::SocketAddr;

use futures::{sink::SinkExt, stream::StreamExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler));

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
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {}...", who);
    } else {
        println!("Could not send ping {}!", who);
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }


    let (mut sender, mut receiver) = socket.split();

    // resv_taskからsend_taskにメッセージを送信するためのチャネル
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    // メッセージを受け取るタスクを起動
    let recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;

            if let Message::Text(text) = msg {
                // メッセージを送信するタスクにメッセージを送信
                println!("message: {:?} from : {:?}", text, who);
                tx.send(text).await.unwrap();
            } else if let Message::Close(_) = msg {
                // Closeメッセージを受け取ったら、メッセージを送信するタスクにCloseメッセージを送信
                println!("close: from : {:?}", who);
                tx.send("close".to_string()).await.unwrap();
                break;
            }
        }
        cnt
    });
    
    // メッセージを送信するタスクを起動
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if msg == "close" {
                // メッセージを受け取るタスクからCloseメッセージを受け取ったら、Closeメッセージを送信して終了
                sender.send(Message::Close(None)).await.unwrap();
                break;
            } else {
                // メッセージを受け取るタスクからメッセージを受け取ったら、メッセージを送信
                sender.send(Message::Text(msg)).await.unwrap();
            }
        }
    });

    // tokio::selct!を使って、メッセージを受け取るタスクとメッセージを送信するタスクのどちらかが終了するまで待つ
    tokio::select! {
        _ = recv_task => {
            println!("end resc_task");
        }
        _ = send_task => {
            println!("{} sent a message", who);
        }
    }

    println!("Websocket context {} destroyed", who);
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {} sent str: {:?}", who, t);
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {} somehow sent close message without CloseFrame", who);
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {} sent pong with {:?}", who, v);
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            println!(">>> {} sent ping with {:?}", who, v);
        }
    }
    ControlFlow::Continue(())
}

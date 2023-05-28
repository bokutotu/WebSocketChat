use axum::{
    extract::connect_info::ConnectInfo,
    response::IntoResponse,
};

use std::net::SocketAddr;

/// 全usersを取得する
pub async fn get_all_users(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    println!("get_all_users: {:?}", addr);
    "get".to_string();
}

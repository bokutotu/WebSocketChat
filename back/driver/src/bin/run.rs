use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
};

use driver::user::get_all_users;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(get_all_users));
    let addr = SocketAddr::from(([127, 0, 0, 1], 1919));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

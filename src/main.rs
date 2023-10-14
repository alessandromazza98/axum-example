use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("it should not fail here");
}

async fn hello_handler() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

#[derive(Serialize)]
struct Message {
    message: String,
}

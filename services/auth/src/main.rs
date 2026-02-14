use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("auth-api listening on :8001");
    serve(listener, app).await.unwrap();
}
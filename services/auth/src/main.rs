use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;
use tokio::signal;

async fn health() -> &'static str {
    "ok"
}

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    let mut sigterm =
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");

    tokio::select! {
        _ = ctrl_c => {}
        _ = sigterm.recv() => {}
    }

    println!("shutdown signal received");
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("auth-api listening on :8001");
    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
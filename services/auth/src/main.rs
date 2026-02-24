mod config;
mod handlers;
mod providers;
mod routes;
mod services;
mod stores;
mod types;

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::signal;

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler");

    tokio::select! {
        _ = ctrl_c => {}
        _ = sigterm.recv() => {}
    }

    tracing::info!("shutdown signal received");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "blog_auth_api=debug,info".into()),
        )
        .init();

    let _ = dotenvy::dotenv();

    let config = config::AppConfig::from_env();

    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let redis_client =
        redis::Client::open(config.redis_url.as_str()).expect("invalid redis URL");
    let redis: redis::aio::MultiplexedConnection = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("failed to connect to redis");

    let state = config::AppState {
        db,
        redis,
        config: Arc::new(config),
        http: reqwest::Client::new(),
    };

    let app = routes::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8001").await.unwrap();
    tracing::info!("auth-api listening on :8001");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
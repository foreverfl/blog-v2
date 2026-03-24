mod auth;
mod config;
mod handlers;
mod repositories;
mod routes;
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
                .unwrap_or_else(|_| "blog_rust_api=debug,info".into()),
        )
        .init();

    let _ = dotenvy::dotenv();

    let config = config::AppConfig::from_env();

    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_config = if let Ok(endpoint) = std::env::var("S3_ENDPOINT") {
        aws_sdk_s3::config::Builder::from(&aws_config)
            .endpoint_url(endpoint)
            .force_path_style(true)
            .build()
    } else {
        aws_sdk_s3::config::Builder::from(&aws_config).build()
    };
    let s3 = aws_sdk_s3::Client::from_conf(s3_config);

    let state = config::AppState {
        db,
        config: Arc::new(config),
        s3,
    };

    let app = routes::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8002").await.unwrap();
    tracing::info!("rust-api listening on :8002");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

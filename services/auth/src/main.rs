mod config;
mod handlers;
mod providers;
mod routes;
mod services;
mod stores;
mod types;

use std::sync::Arc;
use std::time::Duration;

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    let otlp_exporter = SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("failed to build otlp span exporter");
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_resource(
            Resource::builder()
                .with_service_name("blog-auth-api")
                .build(),
        )
        .build();
    let tracer = tracer_provider.tracer("blog-auth-api");

    // Without a registered propagator, extract() in the request layer is a no-op
    // and browser traceparent headers are silently ignored
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "blog_auth_api=debug,info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(true),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
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
    let redis = redis::aio::ConnectionManager::new(redis_client)
        .await
        .expect("failed to connect to redis");

    tokio::spawn(stores::redis::run_health_loop(
        redis.clone(),
        Duration::from_secs(30),
    ));

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

    // Flush spans still buffered in the batch exporter
    let _ = tracer_provider.shutdown();
}
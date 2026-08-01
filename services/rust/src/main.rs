mod auth;
mod config;
mod handlers;
mod routes;
mod services;
mod stores;
mod types;

use std::sync::Arc;

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
                .with_service_name("blog-rust-api")
                .build(),
        )
        .build();
    let tracer = tracer_provider.tracer("blog-rust-api");

    // Without a registered propagator, extract() in the request layer is a no-op
    // and browser traceparent headers are silently ignored
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "blog_rust_api=debug,info".into()),
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

    let redis = redis::Client::open(config.redis_url.as_str())
        .expect("failed to create redis client");

    let state = config::AppState {
        db,
        config: Arc::new(config),
        s3,
        redis,
    };

    let app = routes::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8002").await.unwrap();
    tracing::info!("rust-api listening on :8002");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // Flush spans still buffered in the batch exporter
    let _ = tracer_provider.shutdown();
}

mod config;
mod error;

use axum::{response::IntoResponse, routing::get, Router};
use config::Config;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    info!("Starting LambdaDrop on port {}", config.port);

    tokio::fs::create_dir_all(&config.storage_path).await?;
    info!("Storage path: {}", &config.storage_path);

    let app = Router::new().route("/health", get(health_handler));

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Server listening on http://localhost:{}", config.port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    "OK"
}
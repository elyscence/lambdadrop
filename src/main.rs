use axum::{response::IntoResponse, routing::get, Router};
use std::sync::Arc;
use tower_http::cors::{Cors, CorsLayer};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, error};

mod config;
mod error;
mod models;
mod db;
mod utils;

mod handlers;
//mod routes;

use config::Config;

#[derive(Clone)]
struct AppState {
    pub pool: sqlx::SqlitePool,
    pub config: Config,
}

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

    let pool = db::sqlite::create_pool(&config.database_url).await?;
    info!("Database connected: {}", config.database_url);

    let state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Server listening on http://localhost:{}", config.port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_handler() -> impl IntoResponse {
    "OK"
}
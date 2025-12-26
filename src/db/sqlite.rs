use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, sqlite::SqliteConnectOptions};
use anyhow::Result;
use std::path::Path;
use tokio::fs;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    let db_path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(database_url);

    if let Some(parent) = Path::new(db_path).parent() {
        fs::create_dir_all(parent).await?;
    }

    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::query(include_str!("../../migrations/0101_initial.sql"))
        .execute(&pool)
        .await?;

    // Performance tricks
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;
    
    sqlx::query("PRAGMA synchronous=NORMAL")
        .execute(&pool)
        .await?;
    
    sqlx::query("PRAGMA foreign_keys=ON")
        .execute(&pool)
        .await?;

    Ok(pool)
}
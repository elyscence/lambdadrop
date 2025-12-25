use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
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

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
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
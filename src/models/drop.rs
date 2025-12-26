use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Drop {
    pub id: String,
    pub filename: String,
    pub encrypted_path: String,
    pub mime_type: Option<String>,
    pub size_bytes: i64,
    
    #[sqlx(rename = "nonce")]
    pub nonce_bytes: Vec<u8>,
    
    pub is_text: bool,
    
    pub created_at: i64,
    pub expires_at: i64,

    pub max_downloads: Option<i32>,
    pub download_count: i32,
    
    pub burn_after_read: bool,
}

#[derive(Debug, Deserialize)]
pub struct UploadRequest {
    pub filename: String,
    pub data: String,
    pub nonce: String,
    
    #[serde(default)]
    pub is_text: bool,
    
    pub mime_type: Option<String>,
    
    pub expires_in: Option<i64>,
    pub max_downloads: Option<i32>,
    
    #[serde(default)]
    pub burn_after_read: bool,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadResponse {
    pub filename: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub nonce: String,
    pub data: String,
}

#[derive(Debug, Serialize)]
pub struct DropInfo {
    pub filename: String,
    pub size_bytes: i64,
    pub mime_type: Option<String>,
    pub expires_at: i64,
    pub is_available: bool,
    pub downloads_left: Option<i32>,
}
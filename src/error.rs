use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    InvalidInput(String),
    FileTooLarge,
    StorageError(std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Drop not found or expired".to_string())
            }
            AppError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::FileTooLarge => {
                (StatusCode::PAYLOAD_TOO_LARGE, "File too large".to_string())
            }
            AppError::StorageError(e) => {
                error!("Storage error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Storage error".to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::StorageError(err)
    }
}
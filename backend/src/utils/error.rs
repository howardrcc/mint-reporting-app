use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Query timeout")]
    QueryTimeout,
    
    #[error("File upload error: {0}")]
    FileUpload(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", self.to_string())
            }
            AppError::Serialization(ref e) => {
                tracing::error!("Serialization error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "SERIALIZATION_ERROR", self.to_string())
            }
            AppError::Io(ref e) => {
                tracing::error!("IO error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "IO_ERROR", self.to_string())
            }
            AppError::Anyhow(ref e) => {
                tracing::error!("Anyhow error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "ANYHOW_ERROR", self.to_string())
            }
            AppError::Csv(ref e) => {
                tracing::error!("CSV error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "CSV_ERROR", self.to_string())
            }
            AppError::Validation(ref msg) => {
                tracing::warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }
            AppError::NotFound(ref msg) => {
                tracing::warn!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone())
            }
            AppError::Unauthorized(ref msg) => {
                tracing::warn!("Unauthorized: {}", msg);
                (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone())
            }
            AppError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg.clone())
            }
            AppError::BadRequest(ref msg) => {
                tracing::warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone())
            }
            AppError::QueryTimeout => {
                tracing::warn!("Query timeout");
                (StatusCode::REQUEST_TIMEOUT, "QUERY_TIMEOUT", "Query execution timed out".to_string())
            }
            AppError::FileUpload(ref msg) => {
                tracing::error!("File upload error: {}", msg);
                (StatusCode::BAD_REQUEST, "FILE_UPLOAD_ERROR", msg.clone())
            }
            AppError::Cache(ref msg) => {
                tracing::warn!("Cache error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "CACHE_ERROR", msg.clone())
            }
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            code: Some(error_type.to_string()),
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

impl AppError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn file_upload(msg: impl Into<String>) -> Self {
        Self::FileUpload(msg.into())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_creation() {
        let error = AppError::validation("Invalid input");
        assert_eq!(error.to_string(), "Validation error: Invalid input");

        let error = AppError::not_found("Resource not found");
        assert_eq!(error.to_string(), "Not found: Resource not found");
    }

    #[test]
    fn test_error_response_serialization() {
        let error_response = ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Invalid input".to_string(),
            code: Some("VALIDATION_ERROR".to_string()),
            details: None,
        };

        let json = serde_json::to_string(&error_response).unwrap();
        assert!(json.contains("VALIDATION_ERROR"));
        assert!(json.contains("Invalid input"));
    }
}
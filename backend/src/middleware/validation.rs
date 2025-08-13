use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use serde_json::json;
use tracing::warn;

/// Middleware for validating request content types and sizes
pub async fn validation_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let content_length = request
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok());

    // Check content length limits (1GB max)
    if let Some(length) = content_length {
        if length > 1024 * 1024 * 1024 {
            warn!("Request too large: {} bytes", length);
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }
    }

    // Check for required headers on certain endpoints
    let uri = request.uri().path();
    if uri.starts_with("/api/data/upload") {
        let content_type = request
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());

        if let Some(ct) = content_type {
            if !ct.starts_with("multipart/form-data") {
                warn!("Invalid content type for upload: {}", ct);
                return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
            }
        }
    }

    Ok(next.run(request).await)
}

/// Validate JSON request bodies
pub fn validate_json_request<T>(json: &str) -> Result<T, StatusCode>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str(json).map_err(|e| {
        warn!("JSON validation error: {}", e);
        StatusCode::BAD_REQUEST
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[test]
    fn test_json_validation() {
        let valid_json = r#"{"name": "test", "value": 42}"#;
        let result: Result<TestStruct, StatusCode> = validate_json_request(valid_json);
        assert!(result.is_ok());

        let invalid_json = r#"{"name": "test", "invalid": true}"#;
        let result: Result<TestStruct, StatusCode> = validate_json_request(invalid_json);
        assert!(result.is_err());
    }
}
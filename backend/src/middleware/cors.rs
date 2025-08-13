use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

/// Create CORS layer for the application
pub fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_credentials(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_layer_creation() {
        let cors_layer = create_cors_layer();
        // Just ensure we can create the layer without panicking
        assert!(format!("{:?}", cors_layer).contains("CorsLayer"));
    }
}
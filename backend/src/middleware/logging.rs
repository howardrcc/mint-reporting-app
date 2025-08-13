use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::{info, warn, error};
use std::time::Instant;

/// Logging middleware for HTTP requests
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start_time = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    // Get client IP if available
    let client_ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("unknown");

    info!(
        method = %method,
        uri = %uri,
        version = ?version,
        client_ip = %client_ip,
        "Request started"
    );

    let response = next.run(request).await;
    
    let duration = start_time.elapsed();
    let status = response.status();
    
    match status {
        status if status.is_success() => {
            info!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "Request completed successfully"
            );
        }
        status if status.is_client_error() => {
            warn!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "Request completed with client error"
            );
        }
        status if status.is_server_error() => {
            error!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "Request completed with server error"
            );
        }
        _ => {
            info!(
                method = %method,
                uri = %uri,
                status = %status.as_u16(),
                duration_ms = %duration.as_millis(),
                "Request completed"
            );
        }
    }

    response
}
pub mod database;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;
pub mod utils;

use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::TraceLayer,
};

use crate::{
    handlers::{dashboard, data, analytics, websocket, system},
    middleware::cors::create_cors_layer,
    database::DatabasePool,
    services::file_processor::FileProcessor,
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DatabasePool,
    pub file_processor: FileProcessor,
}

/// Create the main application router with all routes and middleware
pub fn create_app() -> Router<AppState> {
    // Configure CORS
    let cors = create_cors_layer();

    // Build the router with all routes
    Router::new()
        // Health check
        .route("/health", get(system::health_check))
        
        // Data management routes
        .route("/api/data/upload", post(data::upload_data))
        .route("/api/data/sources", get(data::list_sources))
        .route("/api/data/sources/:id", delete(data::delete_source))
        .route("/api/data/schema/:id", get(data::get_schema))
        .route("/api/data/preview/:id", post(data::preview_data))
        
        // Dashboard routes
        .route("/api/dashboard/configs", get(dashboard::list_configs))
        .route("/api/dashboard/configs", post(dashboard::save_config))
        .route("/api/dashboard/configs/:id", put(dashboard::update_config))
        .route("/api/dashboard/configs/:id", delete(dashboard::delete_config))
        
        // Analytics routes
        .route("/api/analytics/query", post(analytics::execute_query))
        .route("/api/analytics/aggregate", post(analytics::run_aggregation))
        .route("/api/analytics/metrics/:id", get(analytics::get_metrics))
        .route("/api/analytics/export", post(analytics::export_data))
        
        // System routes
        .route("/api/system/health", get(system::health_check))
        .route("/api/system/stats", get(system::get_stats))
        .route("/api/system/optimize", post(system::optimize_database))
        
        // WebSocket route
        .route("/ws", get(websocket::websocket_handler))
        
        // Middleware stack
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(cors)
                .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)) // 1GB max upload
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_app() {
        let app = create_app();
        // Basic test to ensure the app can be created
        assert!(!format!("{:?}", app).is_empty());
    }
}
use axum::{extract::{State, Path}, response::Json, http::StatusCode};
use tracing::{debug, info};

use crate::{
    database::queries::DashboardQueries,
    handlers::data::AppState,
    models::{DashboardConfig, CreateDashboardRequest, UpdateDashboardRequest},
    utils::error::{AppError, AppResult},
};

/// List all dashboard configurations
pub async fn list_configs(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<DashboardConfig>>> {
    debug!("Listing all dashboard configurations");

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    let configs = DashboardQueries::list_all(&conn_guard)?;

    Ok(Json(configs))
}

/// Save a new dashboard configuration
pub async fn save_config(
    State(state): State<AppState>,
    Json(request): Json<CreateDashboardRequest>,
) -> AppResult<Json<DashboardConfig>> {
    info!("Creating new dashboard configuration: {}", request.name);

    let config = DashboardConfig::new(
        uuid::Uuid::new_v4().to_string(),
        request.name,
    )
    .with_layout(request.layout)
    .with_data_source(request.data_source_id.unwrap_or_default())
    .with_refresh_interval(request.refresh_interval.unwrap_or(30));

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    DashboardQueries::create(&conn_guard, &config)?;

    info!("Dashboard configuration created successfully: {}", config.id);
    Ok(Json(config))
}

/// Update an existing dashboard configuration
pub async fn update_config(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateDashboardRequest>,
) -> AppResult<Json<DashboardConfig>> {
    info!("Updating dashboard configuration: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    // Get existing config
    let mut config = DashboardQueries::get_by_id(&conn_guard, &id)?
        .ok_or_else(|| AppError::not_found(format!("Dashboard configuration not found: {}", id)))?;

    // Update fields if provided
    if let Some(name) = request.name {
        config.name = name;
    }
    if let Some(layout) = request.layout {
        config.layout = layout;
    }
    if let Some(filters) = request.filters {
        config.filters = Some(filters);
    }
    if let Some(data_source_id) = request.data_source_id {
        config.data_source_id = Some(data_source_id);
    }
    if let Some(refresh_interval) = request.refresh_interval {
        config.refresh_interval = Some(refresh_interval);
    }

    config.updated_at = chrono::Utc::now();

    // Save updated config
    DashboardQueries::update(&conn_guard, &config)?;

    info!("Dashboard configuration updated successfully: {}", id);
    Ok(Json(config))
}

/// Delete a dashboard configuration
pub async fn delete_config(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    info!("Deleting dashboard configuration: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let deleted = DashboardQueries::delete(&conn_guard, &id)?;
    
    if deleted {
        info!("Dashboard configuration deleted successfully: {}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(format!("Dashboard configuration not found: {}", id)))
    }
}
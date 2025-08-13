use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use std::collections::HashMap;
use tracing::{debug, error, info};

use crate::{
    database::{queries::DataSourceQueries, DatabasePool},
    models::{DataSource, DataPreviewRequest, DataPreviewResponse},
    services::file_processor::FileProcessor,
    utils::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DatabasePool,
    pub file_processor: FileProcessor,
}

/// Upload a data file
pub async fn upload_data(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<DataSource>> {
    info!("Starting file upload");

    let mut file_name = None;
    let mut file_data = None;

    // Process multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::file_upload(format!("Failed to read multipart field: {}", e))
    })? {
        let name = field.name().unwrap_or("unknown").to_string();
        
        match name.as_str() {
            "file" => {
                file_name = field.file_name().map(|s| s.to_string());
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::file_upload(format!("Failed to read file data: {}", e))
                })?);
            }
            _ => {
                debug!("Ignoring unknown field: {}", name);
            }
        }
    }

    let file_name = file_name.ok_or_else(|| AppError::bad_request("No file provided"))?;
    let file_data = file_data.ok_or_else(|| AppError::bad_request("No file data provided"))?;

    if file_data.is_empty() {
        return Err(AppError::bad_request("Empty file provided"));
    }

    info!("Processing uploaded file: {} ({} bytes)", file_name, file_data.len());

    // Process the file
    let data_source = state.file_processor.process_file(
        file_name,
        file_data.to_vec(),
    ).await?;

    // Save to database
    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    DataSourceQueries::create(&conn_guard, &data_source)?;

    info!("File upload completed successfully: {}", data_source.id);
    Ok(Json(data_source))
}

/// List all data sources
pub async fn list_sources(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<DataSource>>> {
    debug!("Listing all data sources");

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    let sources = DataSourceQueries::list_all(&conn_guard)?;

    Ok(Json(sources))
}

/// Delete a data source
pub async fn delete_source(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<StatusCode> {
    info!("Deleting data source: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let deleted = DataSourceQueries::delete(&conn_guard, &id)?;
    
    if deleted {
        // Also delete the actual table if it exists
        let table_name = format!("data_source_{}", id.replace('-', "_"));
        if let Err(e) = conn_guard.execute(&format!("DROP TABLE IF EXISTS {}", table_name), []) {
            error!("Failed to drop table {}: {:?}", table_name, e);
        }
        
        info!("Data source deleted successfully: {}", id);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(format!("Data source not found: {}", id)))
    }
}

/// Get schema for a data source
pub async fn get_schema(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<DataSource>> {
    debug!("Getting schema for data source: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let data_source = DataSourceQueries::get_by_id(&conn_guard, &id)?
        .ok_or_else(|| AppError::not_found(format!("Data source not found: {}", id)))?;

    Ok(Json(data_source))
}

/// Preview data from a data source
pub async fn preview_data(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    Json(request): Json<DataPreviewRequest>,
) -> AppResult<Json<DataPreviewResponse>> {
    debug!("Previewing data for source: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    // Verify data source exists
    let data_source = DataSourceQueries::get_by_id(&conn_guard, &id)?
        .ok_or_else(|| AppError::not_found(format!("Data source not found: {}", id)))?;

    let table_name = format!("data_source_{}", id.replace('-', "_"));
    let limit = request.limit.unwrap_or(1000).min(10000); // Max 10k rows for preview
    let offset = request.offset.unwrap_or(0);

    // Build query
    let mut query = format!("SELECT * FROM {} LIMIT {} OFFSET {}", table_name, limit, offset);
    
    // Add filters if provided
    if let Some(filters) = &request.filters {
        if let Some(filter_obj) = filters.as_object() {
            let mut conditions = Vec::new();
            for (field, value) in filter_obj {
                if let Some(str_value) = value.as_str() {
                    conditions.push(format!("{} LIKE '%{}%'", field, str_value));
                } else if let Some(num_value) = value.as_f64() {
                    conditions.push(format!("{} = {}", field, num_value));
                }
            }
            if !conditions.is_empty() {
                query = format!("SELECT * FROM {} WHERE {} LIMIT {} OFFSET {}", 
                    table_name, conditions.join(" AND "), limit, offset);
            }
        }
    }

    debug!("Executing preview query: {}", query);

    // Execute query
    let mut stmt = conn_guard.prepare(&query)?;
    let mut rows = stmt.query([])?;
    
    let column_count = stmt.column_count();
    let columns: Vec<String> = (0..column_count)
        .map(|i| stmt.column_name(i).unwrap_or("unknown").to_string())
        .collect();

    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let mut row_data = Vec::new();
        for i in 0..column_count {
            let value = match row.get_ref(i)? {
                duckdb::types::ValueRef::Null => serde_json::Value::Null,
                duckdb::types::ValueRef::Integer(n) => serde_json::Value::Number(n.into()),
                duckdb::types::ValueRef::Real(f) => serde_json::Value::Number(
                    serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))
                ),
                duckdb::types::ValueRef::Text(s) => serde_json::Value::String(String::from_utf8_lossy(s).to_string()),
                duckdb::types::ValueRef::Blob(_) => serde_json::Value::String("BLOB".to_string()),
            };
            row_data.push(value);
        }
        data.push(row_data);
    }

    let response = DataPreviewResponse {
        columns,
        data: data.clone(),
        total_rows: data_source.row_count,
        preview_rows: data.len(),
    };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabasePool;
    use crate::services::file_processor::FileProcessor;
    use tempfile::NamedTempFile;

    async fn create_test_state() -> AppState {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db_pool = DatabasePool::new(db_path).unwrap();
        let file_processor = FileProcessor::new(db_pool.clone());
        
        AppState {
            db_pool,
            file_processor,
        }
    }

    #[tokio::test]
    async fn test_list_sources_empty() {
        let state = create_test_state().await;
        
        // Initialize database with tables
        let conn = state.db_pool.get_connection();
        let conn_guard = conn.lock().await;
        conn_guard.execute_batch("
            CREATE TABLE data_sources (
                id VARCHAR PRIMARY KEY,
                name VARCHAR NOT NULL,
                type VARCHAR NOT NULL,
                file_path VARCHAR,
                schema_info TEXT,
                row_count BIGINT DEFAULT 0,
                size_bytes BIGINT DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        ").unwrap();
        drop(conn_guard);

        let result = list_sources(State(state)).await.unwrap();
        assert_eq!(result.0.len(), 0);
    }
}
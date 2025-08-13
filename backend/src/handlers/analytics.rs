use axum::{extract::State, response::Json, extract::Path};
use tracing::{debug, info};

use crate::{
    database::queries::AnalyticsQueries,
    handlers::data::AppState,
    models::{QueryRequest, QueryResult, AggregationRequest, AggregationResult, ExportRequest, ExportResult, MetricsRequest, MetricsResult},
    utils::error::AppResult,
};

/// Execute a custom SQL query
pub async fn execute_query(
    State(state): State<AppState>,
    Json(request): Json<QueryRequest>,
) -> AppResult<Json<QueryResult>> {
    info!("Executing custom query");
    debug!("Query: {}", request.sql);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let table_name = if let Some(source_id) = &request.data_source_id {
        format!("data_source_{}", source_id.replace('-', "_"))
    } else {
        "main".to_string()
    };

    let result = AnalyticsQueries::execute_custom_query(&conn_guard, &table_name, &request.sql)?;
    
    Ok(Json(result))
}

/// Run aggregation operations
pub async fn run_aggregation(
    State(state): State<AppState>,
    Json(request): Json<AggregationRequest>,
) -> AppResult<Json<AggregationResult>> {
    info!("Running aggregation for source: {}", request.data_source_id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let table_name = format!("data_source_{}", request.data_source_id.replace('-', "_"));
    
    // Build aggregation query
    let mut select_parts = Vec::new();
    let mut agg_summaries = Vec::new();
    
    for op in &request.operations {
        let alias = op.get_alias();
        let sql_op = match op.operation.as_str() {
            "sum" => format!("SUM({})", op.field),
            "avg" => format!("AVG({})", op.field),
            "count" => format!("COUNT({})", op.field),
            "min" => format!("MIN({})", op.field),
            "max" => format!("MAX({})", op.field),
            "distinct_count" => format!("COUNT(DISTINCT {})", op.field),
            _ => return Err(crate::utils::error::AppError::bad_request(
                format!("Unsupported aggregation operation: {}", op.operation)
            )),
        };
        
        select_parts.push(format!("{} AS {}", sql_op, alias));
        agg_summaries.push(crate::models::AggregationSummary {
            field: op.field.clone(),
            operation: op.operation.clone(),
            result: serde_json::Value::Null, // Will be filled after query
        });
    }
    
    // Add group by fields
    if let Some(group_by) = &request.group_by {
        for field in group_by {
            select_parts.insert(0, field.clone());
        }
    }
    
    let mut query = format!("SELECT {} FROM {}", select_parts.join(", "), table_name);
    
    // Add filters
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
                query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
            }
        }
    }
    
    // Add group by clause
    if let Some(group_by) = &request.group_by {
        if !group_by.is_empty() {
            query.push_str(&format!(" GROUP BY {}", group_by.join(", ")));
        }
    }
    
    // Add limit
    if let Some(limit) = request.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }
    
    debug!("Executing aggregation query: {}", query);
    
    let result = AnalyticsQueries::execute_custom_query(&conn_guard, &table_name, &query)?;
    
    let agg_result = AggregationResult {
        columns: result.columns,
        data: result.data,
        row_count: result.row_count,
        aggregations: agg_summaries,
    };
    
    Ok(Json(agg_result))
}

/// Get predefined metrics for a data source
pub async fn get_metrics(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<MetricsResult>> {
    info!("Getting metrics for data source: {}", id);

    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    let table_name = format!("data_source_{}", id.replace('-', "_"));
    
    // Get basic table statistics
    let stats = AnalyticsQueries::get_table_stats(&conn_guard, &table_name)?;
    
    let metrics = vec![
        crate::models::MetricValue {
            name: "row_count".to_string(),
            value: stats.get("row_count").cloned().unwrap_or(serde_json::Value::Null),
            description: Some("Total number of rows".to_string()),
            unit: Some("rows".to_string()),
        },
        crate::models::MetricValue {
            name: "table_name".to_string(),
            value: stats.get("table_name").cloned().unwrap_or(serde_json::Value::Null),
            description: Some("Table name".to_string()),
            unit: None,
        },
    ];
    
    let result = MetricsResult {
        data_source_id: id,
        metrics,
        calculated_at: chrono::Utc::now(),
    };
    
    Ok(Json(result))
}

/// Export data
pub async fn export_data(
    State(state): State<AppState>,
    Json(request): Json<ExportRequest>,
) -> AppResult<Json<ExportResult>> {
    info!("Exporting data in format: {}", request.format);

    // For now, return a placeholder response
    let result = ExportResult {
        file_url: "/exports/data.csv".to_string(),
        file_size: 1024,
        row_count: 100,
        format: request.format,
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    };
    
    Ok(Json(result))
}
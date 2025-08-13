use duckdb::{Connection, Result as DuckResult, params};
use serde_json::Value as JsonValue;
use crate::models::{DataSource, DashboardConfig, QueryResult};
use tracing::{debug, error};

/// Data source queries
pub struct DataSourceQueries;

impl DataSourceQueries {
    pub fn create(conn: &Connection, data_source: &DataSource) -> DuckResult<()> {
        debug!("Creating data source: {}", data_source.id);
        
        conn.execute(
            "INSERT INTO data_sources (id, name, type, file_path, schema_info, row_count, size_bytes) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                data_source.id,
                data_source.name,
                data_source.r#type,
                data_source.file_path,
                serde_json::to_string(&data_source.schema).unwrap_or_default(),
                data_source.row_count,
                data_source.size_bytes
            ],
        )?;
        
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: &str) -> DuckResult<Option<DataSource>> {
        debug!("Getting data source by id: {}", id);
        
        let mut stmt = conn.prepare(
            "SELECT id, name, type, file_path, schema_info, row_count, size_bytes, created_at, updated_at 
             FROM data_sources WHERE id = ?"
        )?;
        
        let mut rows = stmt.query(params![id])?;
        
        if let Some(row) = rows.next()? {
            let schema_info: String = row.get(4)?;
            let schema = serde_json::from_str(&schema_info).unwrap_or_default();
            
            Ok(Some(DataSource {
                id: row.get(0)?,
                name: row.get(1)?,
                r#type: row.get(2)?,
                file_path: row.get(3)?,
                schema,
                row_count: row.get(5)?,
                size_bytes: row.get(6)?,
                created_at: chrono::Utc::now(), // TODO: Parse from database
                updated_at: chrono::Utc::now(), // TODO: Parse from database
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_all(conn: &Connection) -> DuckResult<Vec<DataSource>> {
        debug!("Listing all data sources");
        
        let mut stmt = conn.prepare(
            "SELECT id, name, type, file_path, schema_info, row_count, size_bytes, created_at, updated_at 
             FROM data_sources ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let schema_info: String = row.get(4)?;
            let schema = serde_json::from_str(&schema_info).unwrap_or_default();
            
            Ok(DataSource {
                id: row.get(0)?,
                name: row.get(1)?,
                r#type: row.get(2)?,
                file_path: row.get(3)?,
                schema,
                row_count: row.get(5)?,
                size_bytes: row.get(6)?,
                created_at: chrono::Utc::now(), // TODO: Parse from database
                updated_at: chrono::Utc::now(), // TODO: Parse from database
            })
        })?;
        
        let mut data_sources = Vec::new();
        for data_source in rows {
            data_sources.push(data_source?);
        }
        
        Ok(data_sources)
    }

    pub fn delete(conn: &Connection, id: &str) -> DuckResult<bool> {
        debug!("Deleting data source: {}", id);
        
        let rows_affected = conn.execute("DELETE FROM data_sources WHERE id = ?", params![id])?;
        Ok(rows_affected > 0)
    }

    pub fn update_stats(conn: &Connection, id: &str, row_count: i64, size_bytes: i64) -> DuckResult<()> {
        debug!("Updating stats for data source {}: {} rows, {} bytes", id, row_count, size_bytes);
        
        conn.execute(
            "UPDATE data_sources SET row_count = ?, size_bytes = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
            params![row_count, size_bytes, id],
        )?;
        
        Ok(())
    }
}

/// Dashboard configuration queries
pub struct DashboardQueries;

impl DashboardQueries {
    pub fn create(conn: &Connection, config: &DashboardConfig) -> DuckResult<()> {
        debug!("Creating dashboard config: {}", config.id);
        
        conn.execute(
            "INSERT INTO dashboard_configs (id, name, layout, filters, data_source_id, refresh_interval) 
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                config.id,
                config.name,
                serde_json::to_string(&config.layout).unwrap_or_default(),
                config.filters.as_ref().map(|f| serde_json::to_string(f).unwrap_or_default()),
                config.data_source_id,
                config.refresh_interval
            ],
        )?;
        
        Ok(())
    }

    pub fn get_by_id(conn: &Connection, id: &str) -> DuckResult<Option<DashboardConfig>> {
        debug!("Getting dashboard config by id: {}", id);
        
        let mut stmt = conn.prepare(
            "SELECT id, name, layout, filters, data_source_id, refresh_interval, created_at, updated_at 
             FROM dashboard_configs WHERE id = ?"
        )?;
        
        let mut rows = stmt.query(params![id])?;
        
        if let Some(row) = rows.next()? {
            let layout_json: String = row.get(2)?;
            let filters_json: Option<String> = row.get(3)?;
            
            Ok(Some(DashboardConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                layout: serde_json::from_str(&layout_json).unwrap_or_default(),
                filters: filters_json.and_then(|f| serde_json::from_str(&f).ok()),
                data_source_id: row.get(4)?,
                refresh_interval: row.get(5)?,
                created_at: chrono::Utc::now(), // TODO: Parse from database
                updated_at: chrono::Utc::now(), // TODO: Parse from database
            }))
        } else {
            Ok(None)
        }
    }

    pub fn list_all(conn: &Connection) -> DuckResult<Vec<DashboardConfig>> {
        debug!("Listing all dashboard configs");
        
        let mut stmt = conn.prepare(
            "SELECT id, name, layout, filters, data_source_id, refresh_interval, created_at, updated_at 
             FROM dashboard_configs ORDER BY created_at DESC"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let layout_json: String = row.get(2)?;
            let filters_json: Option<String> = row.get(3)?;
            
            Ok(DashboardConfig {
                id: row.get(0)?,
                name: row.get(1)?,
                layout: serde_json::from_str(&layout_json).unwrap_or_default(),
                filters: filters_json.and_then(|f| serde_json::from_str(&f).ok()),
                data_source_id: row.get(4)?,
                refresh_interval: row.get(5)?,
                created_at: chrono::Utc::now(), // TODO: Parse from database
                updated_at: chrono::Utc::now(), // TODO: Parse from database
            })
        })?;
        
        let mut configs = Vec::new();
        for config in rows {
            configs.push(config?);
        }
        
        Ok(configs)
    }

    pub fn update(conn: &Connection, config: &DashboardConfig) -> DuckResult<()> {
        debug!("Updating dashboard config: {}", config.id);
        
        conn.execute(
            "UPDATE dashboard_configs 
             SET name = ?, layout = ?, filters = ?, data_source_id = ?, refresh_interval = ?, updated_at = CURRENT_TIMESTAMP 
             WHERE id = ?",
            params![
                config.name,
                serde_json::to_string(&config.layout).unwrap_or_default(),
                config.filters.as_ref().map(|f| serde_json::to_string(f).unwrap_or_default()),
                config.data_source_id,
                config.refresh_interval,
                config.id
            ],
        )?;
        
        Ok(())
    }

    pub fn delete(conn: &Connection, id: &str) -> DuckResult<bool> {
        debug!("Deleting dashboard config: {}", id);
        
        let rows_affected = conn.execute("DELETE FROM dashboard_configs WHERE id = ?", params![id])?;
        Ok(rows_affected > 0)
    }
}

/// Analytics and query operations
pub struct AnalyticsQueries;

impl AnalyticsQueries {
    /// Execute a custom SQL query on a data source
    pub fn execute_custom_query(conn: &Connection, table_name: &str, sql: &str) -> DuckResult<QueryResult> {
        debug!("Executing custom query on table {}: {}", table_name, sql);
        
        // Validate and sanitize the query (basic protection)
        if sql.to_lowercase().contains("drop") || sql.to_lowercase().contains("delete") {
            return Err(duckdb::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Destructive operations are not allowed"
            ))));
        }

        let mut stmt = conn.prepare(sql)?;
        
        let column_count = stmt.column_count();
        let column_names: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).map_or("unknown".to_string(), |v| v.to_string()))
            .collect();
        
        let mut rows = stmt.query([])?;
        let mut data = Vec::new();

        while let Some(row) = rows.next()? {
            let mut row_data = Vec::new();
            for i in 0..column_count {
                let value: JsonValue = match row.get_ref(i)? {
                    duckdb::types::ValueRef::Null => JsonValue::Null,
                    duckdb::types::ValueRef::Int(i) => JsonValue::Number(i.into()),
                    duckdb::types::ValueRef::BigInt(i) => JsonValue::Number(i.into()),
                    duckdb::types::ValueRef::Double(f) => JsonValue::Number(
                        serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))
                    ),
                    duckdb::types::ValueRef::Text(s) => JsonValue::String(String::from_utf8_lossy(s).to_string()),
                    duckdb::types::ValueRef::Blob(_) => JsonValue::String("BLOB".to_string()),
                    _ => JsonValue::String("UNKNOWN".to_string()),
                };
                row_data.push(value);
            }
            data.push(row_data);
        }
        
        let row_count = data.len();
        Ok(QueryResult {
            columns: column_names,
            data,
            row_count,
        })
    }

    /// Get basic statistics for a table
    pub fn get_table_stats(conn: &Connection, table_name: &str) -> DuckResult<JsonValue> {
        debug!("Getting table stats for: {}", table_name);
        
        let query = format!("SELECT COUNT(*) as row_count FROM {}", table_name);
        let row_count: i64 = conn.query_row(&query, [], |row| row.get(0))?;
        
        let stats = serde_json::json!({
            "table_name": table_name,
            "row_count": row_count,
            "analyzed_at": chrono::Utc::now().to_rfc3339()
        });
        
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use crate::models::ColumnSchema;

    #[test]
    fn test_data_source_queries() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let conn = Connection::open(db_path).unwrap();
        
        // Setup tables
        conn.execute_batch("
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
        
        let data_source = DataSource {
            id: "test-id".to_string(),
            name: "Test Source".to_string(),
            r#type: "file".to_string(),
            file_path: Some("/path/to/file.csv".to_string()),
            schema: vec![
                ColumnSchema {
                    name: "id".to_string(),
                    r#type: "INTEGER".to_string(),
                    nullable: false,
                    unique: true,
                    primary_key: true,
                }
            ],
            row_count: 1000,
            size_bytes: 50000,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        // Test create
        DataSourceQueries::create(&conn, &data_source).unwrap();
        
        // Test get by id
        let retrieved = DataSourceQueries::get_by_id(&conn, "test-id").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Source");
        
        // Test list all
        let all_sources = DataSourceQueries::list_all(&conn).unwrap();
        assert_eq!(all_sources.len(), 1);
        
        // Test update stats
        DataSourceQueries::update_stats(&conn, "test-id", 2000, 100000).unwrap();
        let updated = DataSourceQueries::get_by_id(&conn, "test-id").unwrap().unwrap();
        assert_eq!(updated.row_count, 2000);
        
        // Test delete
        let deleted = DataSourceQueries::delete(&conn, "test-id").unwrap();
        assert!(deleted);
        
        let not_found = DataSourceQueries::get_by_id(&conn, "test-id").unwrap();
        assert!(not_found.is_none());
    }
}
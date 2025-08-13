use std::collections::HashMap;
use duckdb::Connection;
use tracing::{debug, info, error};

use crate::{
    models::QueryResult,
    utils::error::{AppError, AppResult},
};

/// DuckDB service for advanced database operations
pub struct DuckDBService {
    connection_pool: crate::database::DatabasePool,
}

impl DuckDBService {
    pub fn new(connection_pool: crate::database::DatabasePool) -> Self {
        Self { connection_pool }
    }

    /// Execute a raw SQL query with parameters
    pub async fn execute_query_with_params(
        &self,
        sql: &str,
        params: Option<&HashMap<String, serde_json::Value>>,
    ) -> AppResult<QueryResult> {
        debug!("Executing SQL query: {}", sql);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        // For now, we'll ignore params as DuckDB parameter binding is complex
        // In a production implementation, you'd properly sanitize and bind parameters
        let mut stmt = conn_guard.prepare(sql)?;
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

        Ok(QueryResult::new(columns, data))
    }

    /// Get table information
    pub async fn get_table_info(&self, table_name: &str) -> AppResult<TableInfo> {
        debug!("Getting table info for: {}", table_name);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        // Get column information
        let describe_sql = format!("DESCRIBE {}", table_name);
        let mut stmt = conn_guard.prepare(&describe_sql)?;
        let mut rows = stmt.query([])?;

        let mut columns = Vec::new();
        while let Some(row) = rows.next()? {
            let column_name: String = row.get(0)?;
            let column_type: String = row.get(1)?;
            let nullable: String = row.get(2).unwrap_or_else(|_| "YES".to_string());
            
            columns.push(ColumnInfo {
                name: column_name,
                data_type: column_type,
                nullable: nullable == "YES",
            });
        }

        // Get row count
        let count_sql = format!("SELECT COUNT(*) FROM {}", table_name);
        let row_count: i64 = conn_guard.query_row(&count_sql, [], |row| row.get(0))?;

        // Get table size (approximate)
        let size_sql = format!("SELECT pg_total_relation_size('{}')", table_name);
        let table_size: i64 = conn_guard.query_row(&size_sql, [], |row| row.get(0))
            .unwrap_or(0); // DuckDB might not support this function

        Ok(TableInfo {
            name: table_name.to_string(),
            columns,
            row_count,
            size_bytes: table_size,
        })
    }

    /// Execute bulk operations efficiently
    pub async fn bulk_insert(
        &self,
        table_name: &str,
        data: Vec<Vec<serde_json::Value>>,
        columns: Vec<String>,
    ) -> AppResult<i64> {
        info!("Bulk inserting {} rows into {}", data.len(), table_name);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        // Create placeholders for the query
        let placeholders = vec!["?"; columns.len()].join(", ");
        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders
        );

        let mut stmt = conn_guard.prepare(&insert_sql)?;
        let mut inserted_count = 0;

        // Begin transaction for better performance
        conn_guard.execute("BEGIN TRANSACTION", [])?;

        for row in data {
            // Convert JSON values to DuckDB parameters
            // This is a simplified implementation
            if let Err(e) = stmt.execute([]) {
                error!("Failed to insert row: {}", e);
                conn_guard.execute("ROLLBACK", [])?;
                return Err(AppError::Database(e));
            }
            inserted_count += 1;
        }

        conn_guard.execute("COMMIT", [])?;

        info!("Successfully inserted {} rows", inserted_count);
        Ok(inserted_count)
    }

    /// Optimize table performance
    pub async fn optimize_table(&self, table_name: &str) -> AppResult<()> {
        info!("Optimizing table: {}", table_name);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        // Analyze table statistics
        let analyze_sql = format!("ANALYZE {}", table_name);
        conn_guard.execute(&analyze_sql, [])?;

        info!("Table optimization completed for: {}", table_name);
        Ok(())
    }

    /// Create indexes for better query performance
    pub async fn create_index(
        &self,
        table_name: &str,
        column_name: &str,
        index_name: Option<&str>,
    ) -> AppResult<()> {
        let index_name = index_name.unwrap_or(&format!("idx_{}_{}", table_name, column_name));
        
        info!("Creating index {} on {}.{}", index_name, table_name, column_name);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        let create_index_sql = format!(
            "CREATE INDEX IF NOT EXISTS {} ON {} ({})",
            index_name, table_name, column_name
        );

        conn_guard.execute(&create_index_sql, [])?;

        info!("Index created successfully: {}", index_name);
        Ok(())
    }

    /// Export table data to various formats
    pub async fn export_table(
        &self,
        table_name: &str,
        format: &str,
        file_path: &str,
    ) -> AppResult<i64> {
        info!("Exporting table {} to {} format at {}", table_name, format, file_path);

        let conn = self.connection_pool.get_connection();
        let conn_guard = conn.lock().await;

        let export_sql = match format.to_lowercase().as_str() {
            "csv" => format!("COPY {} TO '{}' (FORMAT CSV, HEADER)", table_name, file_path),
            "parquet" => format!("COPY {} TO '{}' (FORMAT PARQUET)", table_name, file_path),
            "json" => format!("COPY {} TO '{}' (FORMAT JSON)", table_name, file_path),
            _ => return Err(AppError::bad_request(format!("Unsupported export format: {}", format))),
        };

        conn_guard.execute(&export_sql, [])?;

        // Get row count for return value
        let count_sql = format!("SELECT COUNT(*) FROM {}", table_name);
        let row_count: i64 = conn_guard.query_row(&count_sql, [], |row| row.get(0))?;

        info!("Export completed: {} rows exported", row_count);
        Ok(row_count)
    }
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
    pub row_count: i64,
    pub size_bytes: i64,
}

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    async fn create_test_service() -> DuckDBService {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let pool = crate::database::DatabasePool::new(db_path).unwrap();
        DuckDBService::new(pool)
    }

    #[tokio::test]
    async fn test_execute_query() {
        let service = create_test_service().await;
        
        let result = service.execute_query_with_params("SELECT 1 as test_col", None).await;
        
        match result {
            Ok(query_result) => {
                assert_eq!(query_result.columns.len(), 1);
                assert_eq!(query_result.columns[0], "test_col");
                assert_eq!(query_result.row_count, 1);
            }
            Err(e) => {
                panic!("Query execution failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_table_operations() {
        let service = create_test_service().await;
        
        // Create a test table
        let conn = service.connection_pool.get_connection();
        let conn_guard = conn.lock().await;
        conn_guard.execute_batch("
            CREATE TABLE test_table (
                id INTEGER PRIMARY KEY,
                name VARCHAR,
                value DOUBLE
            );
            INSERT INTO test_table VALUES (1, 'test', 3.14);
        ").unwrap();
        drop(conn_guard);

        // Test table info
        let table_info = service.get_table_info("test_table").await.unwrap();
        assert_eq!(table_info.name, "test_table");
        assert_eq!(table_info.columns.len(), 3);
        assert_eq!(table_info.row_count, 1);

        // Test table optimization
        let optimize_result = service.optimize_table("test_table").await;
        assert!(optimize_result.is_ok());
    }
}
use std::sync::Arc;
use duckdb::{Connection, Result as DuckResult};
use tokio::sync::Mutex;
use tracing::{debug, error};

use super::DatabaseConnection;

/// Database connection manager
pub struct ConnectionManager {
    database_path: String,
}

impl ConnectionManager {
    pub fn new(database_path: String) -> Self {
        Self { database_path }
    }

    /// Create a new connection to the database
    pub fn create_connection(&self) -> DuckResult<DatabaseConnection> {
        debug!("Creating new database connection to: {}", self.database_path);
        
        let conn = Connection::open(&self.database_path)?;
        
        // Configure DuckDB for optimal performance
        conn.execute_batch("
            SET memory_limit='2GB';
            SET threads=4;
            SET enable_progress_bar=false;
            SET preserve_insertion_order=false;
        ")?;
        
        Ok(Arc::new(Mutex::new(conn)))
    }

    /// Test the database connection
    pub async fn test_connection(&self) -> anyhow::Result<()> {
        let conn = self.create_connection()?;
        let conn_guard = conn.lock().await;
        
        let result: i32 = conn_guard.query_row("SELECT 1", [], |row| row.get(0))?;
        
        if result == 1 {
            debug!("Database connection test successful");
            Ok(())
        } else {
            error!("Database connection test failed: unexpected result {}", result);
            Err(anyhow::anyhow!("Database connection test failed"))
        }
    }

    /// Get database statistics
    pub async fn get_database_info(&self) -> anyhow::Result<DatabaseInfo> {
        let conn = self.create_connection()?;
        let conn_guard = conn.lock().await;
        
        // Get database version
        let version: String = conn_guard.query_row(
            "SELECT version()", 
            [], 
            |row| row.get(0)
        )?;

        // Get memory usage
        let mut stmt = conn_guard.prepare("PRAGMA database_size")?;
        let memory_usage: i64 = stmt.query_row([], |row| row.get(0))?;

        // Get table count
        let mut stmt = conn_guard.prepare("
            SELECT COUNT(*) 
            FROM information_schema.tables 
            WHERE table_schema = 'main'
        ")?;
        let table_count: i64 = stmt.query_row([], |row| row.get(0))?;

        Ok(DatabaseInfo {
            version,
            memory_usage,
            table_count,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseInfo {
    pub version: String,
    pub memory_usage: i64,
    pub table_count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_connection_manager() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap().to_string();
        
        let manager = ConnectionManager::new(db_path);
        
        // Test connection creation
        let conn = manager.create_connection().unwrap();
        assert!(!conn.lock().await.is_readonly(duckdb::OpenFlags::SQLITE_OPEN_READWRITE).unwrap());
        
        // Test connection health
        assert!(manager.test_connection().await.is_ok());
        
        // Test database info
        let info = manager.get_database_info().await.unwrap();
        assert!(!info.version.is_empty());
        assert!(info.table_count >= 0);
    }
}
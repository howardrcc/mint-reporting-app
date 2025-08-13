pub mod connection;
pub mod migrations;
pub mod queries;

use std::sync::Arc;
use duckdb::{Connection, Result as DuckResult};
use tokio::sync::Mutex;
use tracing::{info, error};

pub type DatabaseConnection = Arc<Mutex<Connection>>;

/// Initialize the database connection and run migrations
pub async fn init(database_path: &str) -> anyhow::Result<()> {
    info!("Initializing database at: {}", database_path);
    
    let conn = Connection::open(database_path)?;
    
    // Run migrations
    migrations::run_migrations(&conn).await?;
    
    info!("Database initialization completed successfully");
    Ok(())
}

/// Create a new database connection
pub fn create_connection(database_path: &str) -> DuckResult<DatabaseConnection> {
    let conn = Connection::open(database_path)?;
    Ok(Arc::new(Mutex::new(conn)))
}

/// Get a thread-safe database connection pool
#[derive(Clone)]
pub struct DatabasePool {
    connection: DatabaseConnection,
}

impl DatabasePool {
    pub fn new(database_path: &str) -> DuckResult<Self> {
        let connection = create_connection(database_path)?;
        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> DatabaseConnection {
        Arc::clone(&self.connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_database_init() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        let result = init(db_path).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_database_pool() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        
        let pool = DatabasePool::new(db_path).unwrap();
        let conn = pool.get_connection();
        
        // Test basic query
        let conn_guard = conn.lock().await;
        let result: i32 = conn_guard.query_row("SELECT 1", [], |row| row.get(0)).unwrap();
        assert_eq!(result, 1);
    }
}
use duckdb::{Connection, Result as DuckResult};
use tracing::{info, error};

/// Run all database migrations
pub async fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
    info!("Running database migrations");

    // Create migrations table if it doesn't exist
    create_migrations_table(conn)?;

    // Get current migration version
    let current_version = get_current_migration_version(conn)?;
    info!("Current migration version: {}", current_version);

    // Define all migrations
    let migrations = get_migrations();

    // Run pending migrations
    for (version, migration) in migrations.iter() {
        if *version > current_version {
            info!("Running migration {}: {}", version, migration.name);
            
            match run_migration(conn, migration) {
                Ok(_) => {
                    update_migration_version(conn, *version)?;
                    info!("Migration {} completed successfully", version);
                }
                Err(e) => {
                    error!("Migration {} failed: {}", version, e);
                    return Err(anyhow::anyhow!("Migration {} failed: {}", version, e));
                }
            }
        }
    }

    info!("All migrations completed successfully");
    Ok(())
}

fn create_migrations_table(conn: &Connection) -> DuckResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS __migrations (
            version INTEGER PRIMARY KEY,
            applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

fn get_current_migration_version(conn: &Connection) -> DuckResult<i32> {
    match conn.query_row(
        "SELECT MAX(version) FROM __migrations",
        [],
        |row| row.get::<_, Option<i32>>(0),
    ) {
        Ok(Some(version)) => Ok(version),
        Ok(None) => Ok(0),
        Err(_) => Ok(0), // Table doesn't exist yet
    }
}

fn update_migration_version(conn: &Connection, version: i32) -> DuckResult<()> {
    conn.execute(
        "INSERT INTO __migrations (version) VALUES (?)",
        [version],
    )?;
    Ok(())
}

struct Migration {
    name: &'static str,
    sql: &'static str,
}

fn get_migrations() -> Vec<(i32, Migration)> {
    vec![
        (1, Migration {
            name: "Create data_sources table",
            sql: "
                CREATE TABLE data_sources (
                    id VARCHAR PRIMARY KEY,
                    name VARCHAR NOT NULL,
                    type VARCHAR NOT NULL CHECK (type IN ('file', 'database', 'api')),
                    file_path VARCHAR,
                    schema_info JSON,
                    row_count BIGINT DEFAULT 0,
                    size_bytes BIGINT DEFAULT 0,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
            ",
        }),
        (2, Migration {
            name: "Create dashboard_configs table",
            sql: "
                CREATE TABLE dashboard_configs (
                    id VARCHAR PRIMARY KEY,
                    name VARCHAR NOT NULL,
                    layout JSON NOT NULL,
                    filters JSON,
                    data_source_id VARCHAR,
                    refresh_interval INTEGER,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (data_source_id) REFERENCES data_sources(id) ON DELETE SET NULL
                );
            ",
        }),
        (3, Migration {
            name: "Create query_cache table",
            sql: "
                CREATE TABLE query_cache (
                    id VARCHAR PRIMARY KEY,
                    query_hash VARCHAR NOT NULL UNIQUE,
                    query_sql TEXT NOT NULL,
                    result_data JSON,
                    data_source_id VARCHAR,
                    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    expires_at TIMESTAMP,
                    FOREIGN KEY (data_source_id) REFERENCES data_sources(id) ON DELETE CASCADE
                );
                CREATE INDEX idx_query_cache_hash ON query_cache(query_hash);
                CREATE INDEX idx_query_cache_expires ON query_cache(expires_at);
            ",
        }),
        (4, Migration {
            name: "Create analytics_metrics table",
            sql: "
                CREATE TABLE analytics_metrics (
                    id VARCHAR PRIMARY KEY,
                    data_source_id VARCHAR NOT NULL,
                    metric_name VARCHAR NOT NULL,
                    metric_value DOUBLE,
                    metadata JSON,
                    calculated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY (data_source_id) REFERENCES data_sources(id) ON DELETE CASCADE
                );
                CREATE INDEX idx_analytics_metrics_source ON analytics_metrics(data_source_id);
                CREATE INDEX idx_analytics_metrics_name ON analytics_metrics(metric_name);
            ",
        }),
        (5, Migration {
            name: "Create system_stats table",
            sql: "
                CREATE TABLE system_stats (
                    id INTEGER PRIMARY KEY,
                    memory_usage BIGINT,
                    active_connections INTEGER,
                    query_count BIGINT DEFAULT 0,
                    error_count BIGINT DEFAULT 0,
                    recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );
                CREATE INDEX idx_system_stats_recorded ON system_stats(recorded_at);
            ",
        }),
    ]
}

fn run_migration(conn: &Connection, migration: &Migration) -> DuckResult<()> {
    conn.execute_batch(migration.sql)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_migrations() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let conn = Connection::open(db_path).unwrap();
        
        // Run migrations
        let result = run_migrations(&conn).await;
        assert!(result.is_ok());
        
        // Verify tables were created
        let table_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'main'",
            [],
            |row| row.get(0)
        ).unwrap();
        
        assert!(table_count >= 5); // At least our 5 main tables
        
        // Verify migration version
        let version: i32 = conn.query_row(
            "SELECT MAX(version) FROM __migrations",
            [],
            |row| row.get(0)
        ).unwrap();
        
        assert_eq!(version, 5);
    }

    #[tokio::test]
    async fn test_idempotent_migrations() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let conn = Connection::open(db_path).unwrap();
        
        // Run migrations twice
        let result1 = run_migrations(&conn).await;
        let result2 = run_migrations(&conn).await;
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        
        // Version should still be the same
        let version: i32 = conn.query_row(
            "SELECT MAX(version) FROM __migrations",
            [],
            |row| row.get(0)
        ).unwrap();
        
        assert_eq!(version, 5);
    }
}
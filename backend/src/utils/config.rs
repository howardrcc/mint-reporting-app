use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub host: String,
    pub port: u16,
    pub max_upload_size: usize,
    pub query_timeout: u64,
    pub cache_ttl: i64,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn new(database_path: String, host: String, port: u16) -> Self {
        Self {
            database_path,
            host,
            port,
            max_upload_size: 1024 * 1024 * 1024, // 1GB
            query_timeout: 30, // 30 seconds
            cache_ttl: 300, // 5 minutes
            cors_origins: vec!["*".to_string()],
        }
    }

    pub fn from_env() -> Self {
        let database_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "dashboard.db".to_string());
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        let max_upload_size = std::env::var("MAX_UPLOAD_SIZE")
            .unwrap_or_else(|_| "1073741824".to_string()) // 1GB in bytes
            .parse()
            .unwrap_or(1024 * 1024 * 1024);

        let query_timeout = std::env::var("QUERY_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        let cache_ttl = std::env::var("CACHE_TTL")
            .unwrap_or_else(|_| "300".to_string())
            .parse()
            .unwrap_or(300);

        let cors_origins = std::env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "*".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            database_path,
            host,
            port,
            max_upload_size,
            query_timeout,
            cache_ttl,
            cors_origins,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new(
            "test.db".to_string(),
            "localhost".to_string(),
            8080,
        );

        assert_eq!(config.database_path, "test.db");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_upload_size, 1024 * 1024 * 1024);
    }

    #[test]
    fn test_config_from_env() {
        // Test with default values (no env vars set)
        let config = Config::from_env();

        assert_eq!(config.database_path, "dashboard.db");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert_eq!(config.cors_origins, vec!["*"]);
    }
}
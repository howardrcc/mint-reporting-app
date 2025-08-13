use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String, // 'file' | 'database' | 'api'
    pub file_path: Option<String>,
    pub schema: Vec<ColumnSchema>,
    pub row_count: i64,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSchema {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String, // 'INTEGER' | 'DOUBLE' | 'VARCHAR' | 'DATE' | 'TIMESTAMP' | 'BOOLEAN'
    pub nullable: bool,
    pub unique: bool,
    pub primary_key: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataSourceRequest {
    pub name: String,
    pub r#type: String,
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPreviewRequest {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub filters: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPreviewResponse {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
    pub total_rows: i64,
    pub preview_rows: usize,
}

impl DataSource {
    pub fn new(id: String, name: String, r#type: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            r#type,
            file_path: None,
            schema: Vec::new(),
            row_count: 0,
            size_bytes: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn with_schema(mut self, schema: Vec<ColumnSchema>) -> Self {
        self.schema = schema;
        self
    }

    pub fn with_stats(mut self, row_count: i64, size_bytes: i64) -> Self {
        self.row_count = row_count;
        self.size_bytes = size_bytes;
        self.updated_at = Utc::now();
        self
    }
}

impl ColumnSchema {
    pub fn new(name: String, r#type: String) -> Self {
        Self {
            name,
            r#type,
            nullable: true,
            unique: false,
            primary_key: false,
        }
    }

    pub fn with_constraints(mut self, nullable: bool, unique: bool, primary_key: bool) -> Self {
        self.nullable = nullable;
        self.unique = unique;
        self.primary_key = primary_key;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_source_creation() {
        let data_source = DataSource::new(
            "test-id".to_string(),
            "Test Source".to_string(),
            "file".to_string(),
        )
        .with_file_path("/path/to/file.csv".to_string())
        .with_stats(1000, 50000);

        assert_eq!(data_source.id, "test-id");
        assert_eq!(data_source.name, "Test Source");
        assert_eq!(data_source.r#type, "file");
        assert_eq!(data_source.file_path, Some("/path/to/file.csv".to_string()));
        assert_eq!(data_source.row_count, 1000);
        assert_eq!(data_source.size_bytes, 50000);
    }

    #[test]
    fn test_column_schema_creation() {
        let column = ColumnSchema::new("id".to_string(), "INTEGER".to_string())
            .with_constraints(false, true, true);

        assert_eq!(column.name, "id");
        assert_eq!(column.r#type, "INTEGER");
        assert!(!column.nullable);
        assert!(column.unique);
        assert!(column.primary_key);
    }

    #[test]
    fn test_serialization() {
        let data_source = DataSource::new(
            "test-id".to_string(),
            "Test Source".to_string(),
            "file".to_string(),
        );

        let json = serde_json::to_string(&data_source).unwrap();
        let deserialized: DataSource = serde_json::from_str(&json).unwrap();

        assert_eq!(data_source.id, deserialized.id);
        assert_eq!(data_source.name, deserialized.name);
        assert_eq!(data_source.r#type, deserialized.r#type);
    }
}
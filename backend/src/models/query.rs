use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub sql: String,
    pub data_source_id: Option<String>,
    pub params: Option<serde_json::Value>,
    pub cache: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
    pub row_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRequest {
    pub data_source_id: String,
    pub operations: Vec<AggregationOperation>,
    pub group_by: Option<Vec<String>>,
    pub filters: Option<serde_json::Value>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationOperation {
    pub field: String,
    pub operation: String, // 'sum' | 'avg' | 'count' | 'min' | 'max' | 'distinct_count'
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationResult {
    pub columns: Vec<String>,
    pub data: Vec<Vec<serde_json::Value>>,
    pub row_count: usize,
    pub aggregations: Vec<AggregationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationSummary {
    pub field: String,
    pub operation: String,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub data_source_id: Option<String>,
    pub query: Option<String>,
    pub format: String, // 'csv' | 'json' | 'parquet'
    pub filters: Option<serde_json::Value>,
    pub columns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub file_url: String,
    pub file_size: i64,
    pub row_count: i64,
    pub format: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsRequest {
    pub data_source_id: String,
    pub metrics: Vec<String>, // List of predefined metric names
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsResult {
    pub data_source_id: String,
    pub metrics: Vec<MetricValue>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub name: String,
    pub value: serde_json::Value,
    pub description: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCache {
    pub id: String,
    pub query_hash: String,
    pub query_sql: String,
    pub result_data: QueryResult,
    pub data_source_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryError {
    pub message: String,
    pub code: Option<String>,
    pub line: Option<i32>,
    pub column: Option<i32>,
}

impl QueryResult {
    pub fn new(columns: Vec<String>, data: Vec<Vec<serde_json::Value>>) -> Self {
        let row_count = data.len();
        Self {
            columns,
            data,
            row_count,
        }
    }

    pub fn empty() -> Self {
        Self {
            columns: Vec::new(),
            data: Vec::new(),
            row_count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl AggregationOperation {
    pub fn new(field: String, operation: String) -> Self {
        Self {
            field,
            operation,
            alias: None,
        }
    }

    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }

    pub fn get_alias(&self) -> String {
        self.alias.clone().unwrap_or_else(|| {
            format!("{}_{}", self.operation, self.field)
        })
    }
}

impl QueryCache {
    pub fn new(
        query_hash: String,
        query_sql: String,
        result_data: QueryResult,
        data_source_id: Option<String>,
        ttl_seconds: i64,
    ) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::seconds(ttl_seconds);
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            query_hash,
            query_sql,
            result_data,
            data_source_id,
            created_at: now,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_result_creation() {
        let columns = vec!["id".to_string(), "name".to_string()];
        let data = vec![
            vec![serde_json::json!(1), serde_json::json!("Alice")],
            vec![serde_json::json!(2), serde_json::json!("Bob")],
        ];

        let result = QueryResult::new(columns.clone(), data);

        assert_eq!(result.columns, columns);
        assert_eq!(result.row_count, 2);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_empty_query_result() {
        let result = QueryResult::empty();

        assert!(result.columns.is_empty());
        assert_eq!(result.row_count, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_aggregation_operation() {
        let op = AggregationOperation::new("sales".to_string(), "sum".to_string())
            .with_alias("total_sales".to_string());

        assert_eq!(op.field, "sales");
        assert_eq!(op.operation, "sum");
        assert_eq!(op.get_alias(), "total_sales");
    }

    #[test]
    fn test_aggregation_operation_without_alias() {
        let op = AggregationOperation::new("count".to_string(), "avg".to_string());

        assert_eq!(op.get_alias(), "avg_count");
    }

    #[test]
    fn test_query_cache() {
        let result = QueryResult::empty();
        let cache = QueryCache::new(
            "hash123".to_string(),
            "SELECT * FROM table".to_string(),
            result,
            Some("source-1".to_string()),
            300, // 5 minutes
        );

        assert_eq!(cache.query_hash, "hash123");
        assert_eq!(cache.data_source_id, Some("source-1".to_string()));
        assert!(!cache.is_expired());
    }

    #[test]
    fn test_serialization() {
        let request = QueryRequest {
            sql: "SELECT * FROM table".to_string(),
            data_source_id: Some("source-1".to_string()),
            params: None,
            cache: Some(true),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: QueryRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.sql, deserialized.sql);
        assert_eq!(request.data_source_id, deserialized.data_source_id);
    }
}
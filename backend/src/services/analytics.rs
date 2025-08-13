use std::collections::HashMap;
use tracing::{debug, info};

use crate::{
    database::DatabasePool,
    models::{QueryResult, AggregationOperation, MetricValue},
    services::duckdb::DuckDBService,
    utils::error::{AppError, AppResult},
};

/// Analytics service for advanced data analysis
pub struct AnalyticsService {
    duckdb_service: DuckDBService,
}

impl AnalyticsService {
    pub fn new(db_pool: DatabasePool) -> Self {
        Self {
            duckdb_service: DuckDBService::new(db_pool),
        }
    }

    /// Calculate statistical metrics for a dataset
    pub async fn calculate_statistics(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> AppResult<HashMap<String, f64>> {
        info!("Calculating statistics for {}.{}", table_name, column_name);

        let sql = format!(
            "SELECT 
                COUNT({0}) as count,
                AVG({0}) as mean,
                MIN({0}) as min,
                MAX({0}) as max,
                STDDEV({0}) as std_dev,
                PERCENTILE_CONT(0.5) WITHIN GROUP (ORDER BY {0}) as median,
                PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY {0}) as q1,
                PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY {0}) as q3
            FROM {1}
            WHERE {0} IS NOT NULL",
            column_name, table_name
        );

        let result = self.duckdb_service.execute_query_with_params(&sql, None).await?;
        
        if result.data.is_empty() {
            return Ok(HashMap::new());
        }

        let row = &result.data[0];
        let mut stats = HashMap::new();

        if let Some(columns) = result.columns.get(0) {
            for (i, col_name) in result.columns.iter().enumerate() {
                if let Some(value) = row.get(i) {
                    if let Some(num_value) = value.as_f64() {
                        stats.insert(col_name.clone(), num_value);
                    }
                }
            }
        }

        Ok(stats)
    }

    /// Generate time series aggregations
    pub async fn time_series_aggregation(
        &self,
        table_name: &str,
        time_column: &str,
        value_column: &str,
        interval: &str, // 'hour', 'day', 'week', 'month'
        aggregation: &str, // 'sum', 'avg', 'count', 'min', 'max'
    ) -> AppResult<QueryResult> {
        info!("Generating time series aggregation for {}.{} by {}", table_name, value_column, interval);

        let time_trunc = match interval {
            "hour" => format!("date_trunc('hour', {})", time_column),
            "day" => format!("date_trunc('day', {})", time_column),
            "week" => format!("date_trunc('week', {})", time_column),
            "month" => format!("date_trunc('month', {})", time_column),
            _ => return Err(AppError::bad_request(format!("Invalid interval: {}", interval))),
        };

        let agg_func = match aggregation {
            "sum" => format!("SUM({})", value_column),
            "avg" => format!("AVG({})", value_column),
            "count" => format!("COUNT({})", value_column),
            "min" => format!("MIN({})", value_column),
            "max" => format!("MAX({})", value_column),
            _ => return Err(AppError::bad_request(format!("Invalid aggregation: {}", aggregation))),
        };

        let sql = format!(
            "SELECT 
                {} as time_period,
                {} as value
            FROM {}
            WHERE {} IS NOT NULL AND {} IS NOT NULL
            GROUP BY {}
            ORDER BY time_period",
            time_trunc, agg_func, table_name, time_column, value_column, time_trunc
        );

        self.duckdb_service.execute_query_with_params(&sql, None).await
    }

    /// Detect outliers using statistical methods
    pub async fn detect_outliers(
        &self,
        table_name: &str,
        column_name: &str,
        method: &str, // 'iqr', 'zscore'
        threshold: Option<f64>,
    ) -> AppResult<QueryResult> {
        info!("Detecting outliers in {}.{} using {} method", table_name, column_name, method);

        let sql = match method {
            "iqr" => {
                format!(
                    "WITH stats AS (
                        SELECT 
                            PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY {0}) as q1,
                            PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY {0}) as q3
                        FROM {1}
                        WHERE {0} IS NOT NULL
                    ),
                    outlier_bounds AS (
                        SELECT 
                            q1 - 1.5 * (q3 - q1) as lower_bound,
                            q3 + 1.5 * (q3 - q1) as upper_bound
                        FROM stats
                    )
                    SELECT *
                    FROM {1}
                    CROSS JOIN outlier_bounds
                    WHERE {0} < lower_bound OR {0} > upper_bound",
                    column_name, table_name
                )
            },
            "zscore" => {
                let z_threshold = threshold.unwrap_or(3.0);
                format!(
                    "WITH stats AS (
                        SELECT 
                            AVG({0}) as mean,
                            STDDEV({0}) as std_dev
                        FROM {1}
                        WHERE {0} IS NOT NULL
                    )
                    SELECT *,
                           ABS(({0} - stats.mean) / stats.std_dev) as z_score
                    FROM {1}
                    CROSS JOIN stats
                    WHERE ABS(({0} - stats.mean) / stats.std_dev) > {}",
                    column_name, table_name, z_threshold
                )
            },
            _ => return Err(AppError::bad_request(format!("Invalid outlier detection method: {}", method))),
        };

        self.duckdb_service.execute_query_with_params(&sql, None).await
    }

    /// Calculate correlation matrix between numeric columns
    pub async fn correlation_matrix(
        &self,
        table_name: &str,
        columns: Vec<String>,
    ) -> AppResult<HashMap<String, HashMap<String, f64>>> {
        info!("Calculating correlation matrix for {} columns in {}", columns.len(), table_name);

        let mut correlations = HashMap::new();

        for col1 in &columns {
            let mut col1_correlations = HashMap::new();
            
            for col2 in &columns {
                let sql = format!(
                    "SELECT CORR({}, {}) as correlation
                    FROM {}
                    WHERE {} IS NOT NULL AND {} IS NOT NULL",
                    col1, col2, table_name, col1, col2
                );

                let result = self.duckdb_service.execute_query_with_params(&sql, None).await?;
                
                let correlation = if !result.data.is_empty() {
                    result.data[0][0].as_f64().unwrap_or(0.0)
                } else {
                    0.0
                };

                col1_correlations.insert(col2.clone(), correlation);
            }
            
            correlations.insert(col1.clone(), col1_correlations);
        }

        Ok(correlations)
    }

    /// Generate data quality report
    pub async fn data_quality_report(
        &self,
        table_name: &str,
    ) -> AppResult<Vec<DataQualityMetric>> {
        info!("Generating data quality report for {}", table_name);

        // Get table schema
        let table_info = self.duckdb_service.get_table_info(table_name).await?;
        let mut metrics = Vec::new();

        for column in table_info.columns {
            // Calculate null percentage
            let null_sql = format!(
                "SELECT 
                    COUNT(*) as total_rows,
                    COUNT({}) as non_null_rows,
                    (COUNT(*) - COUNT({})) as null_rows,
                    ROUND((COUNT(*) - COUNT({})) * 100.0 / COUNT(*), 2) as null_percentage
                FROM {}",
                column.name, column.name, column.name, table_name
            );

            let null_result = self.duckdb_service.execute_query_with_params(&null_sql, None).await?;
            
            let quality_metric = if !null_result.data.is_empty() {
                let row = &null_result.data[0];
                DataQualityMetric {
                    column_name: column.name.clone(),
                    data_type: column.data_type.clone(),
                    total_rows: row[0].as_i64().unwrap_or(0),
                    null_count: row[2].as_i64().unwrap_or(0),
                    null_percentage: row[3].as_f64().unwrap_or(0.0),
                    unique_count: None, // Could be calculated separately
                    min_length: None,
                    max_length: None,
                }
            } else {
                DataQualityMetric {
                    column_name: column.name.clone(),
                    data_type: column.data_type.clone(),
                    total_rows: 0,
                    null_count: 0,
                    null_percentage: 0.0,
                    unique_count: None,
                    min_length: None,
                    max_length: None,
                }
            };

            metrics.push(quality_metric);
        }

        Ok(metrics)
    }

    /// Calculate moving averages
    pub async fn moving_average(
        &self,
        table_name: &str,
        value_column: &str,
        order_column: &str,
        window_size: i32,
    ) -> AppResult<QueryResult> {
        info!("Calculating {}-period moving average for {}.{}", window_size, table_name, value_column);

        let sql = format!(
            "SELECT *,
                   AVG({}) OVER (
                       ORDER BY {} 
                       ROWS BETWEEN {} PRECEDING AND CURRENT ROW
                   ) as moving_avg
            FROM {}
            ORDER BY {}",
            value_column, order_column, window_size - 1, table_name, order_column
        );

        self.duckdb_service.execute_query_with_params(&sql, None).await
    }
}

#[derive(Debug, Clone)]
pub struct DataQualityMetric {
    pub column_name: String,
    pub data_type: String,
    pub total_rows: i64,
    pub null_count: i64,
    pub null_percentage: f64,
    pub unique_count: Option<i64>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    async fn create_test_service() -> AnalyticsService {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path().to_str().unwrap();
        let db_pool = DatabasePool::new(db_path).unwrap();
        AnalyticsService::new(db_pool)
    }

    #[tokio::test]
    async fn test_statistics_calculation() {
        let service = create_test_service().await;
        
        // Create test data
        let conn = service.duckdb_service.connection_pool.get_connection();
        let conn_guard = conn.lock().await;
        conn_guard.execute_batch("
            CREATE TABLE test_stats (
                id INTEGER,
                value DOUBLE
            );
            INSERT INTO test_stats VALUES 
                (1, 10.0), (2, 20.0), (3, 30.0), (4, 40.0), (5, 50.0);
        ").unwrap();
        drop(conn_guard);

        let stats = service.calculate_statistics("test_stats", "value").await.unwrap();
        
        assert!(stats.contains_key("count"));
        assert!(stats.contains_key("mean"));
        assert_eq!(stats.get("count").copied().unwrap_or(0.0), 5.0);
        assert_eq!(stats.get("mean").copied().unwrap_or(0.0), 30.0);
    }
}
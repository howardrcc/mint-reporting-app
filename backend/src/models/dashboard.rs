use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub id: String,
    pub name: String,
    pub layout: Vec<WidgetLayout>,
    pub filters: Option<serde_json::Value>,
    pub data_source_id: Option<String>,
    pub refresh_interval: Option<i32>, // in seconds
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetLayout {
    pub id: String,
    #[serde(rename = "type")]
    pub widget_type: String, // 'ag-chart' | 'ag-grid' | 'metric' | 'filter'
    pub position: Position,
    pub config: WidgetConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WidgetConfig {
    Chart(AGChartConfig),
    Grid(AGGridConfig),
    Metric(MetricConfig),
    Filter(FilterConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGChartConfig {
    #[serde(rename = "type")]
    pub chart_type: String, // 'line' | 'bar' | 'scatter' | 'pie' | 'donut' | 'area' | 'column'
    pub data: Vec<serde_json::Value>,
    pub series: Vec<AGChartSeries>,
    pub axes: Option<Vec<AGChartAxis>>,
    pub legend: Option<AGChartLegend>,
    pub theme: Option<String>, // 'ag-default' | 'ag-default-dark' | 'ag-material' | 'ag-pastel'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGChartSeries {
    #[serde(rename = "type")]
    pub series_type: String,
    #[serde(rename = "xKey")]
    pub x_key: String,
    #[serde(rename = "yKey")]
    pub y_key: String,
    #[serde(rename = "yName")]
    pub y_name: Option<String>,
    pub stroke: Option<String>,
    pub fill: Option<String>,
    pub marker: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGChartAxis {
    #[serde(rename = "type")]
    pub axis_type: String, // 'category' | 'number' | 'time'
    pub position: String, // 'bottom' | 'left' | 'top' | 'right'
    pub title: Option<AGChartAxisTitle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGChartAxisTitle {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGChartLegend {
    pub enabled: bool,
    pub position: Option<String>, // 'top' | 'bottom' | 'left' | 'right'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGGridConfig {
    #[serde(rename = "columnDefs")]
    pub column_defs: Vec<AGGridColumnDef>,
    #[serde(rename = "rowData")]
    pub row_data: Vec<serde_json::Value>,
    pub pagination: bool,
    #[serde(rename = "paginationPageSize")]
    pub pagination_page_size: i32,
    #[serde(rename = "rowSelection")]
    pub row_selection: String, // 'single' | 'multiple'
    #[serde(rename = "enableRangeSelection")]
    pub enable_range_selection: bool,
    #[serde(rename = "enableCharts")]
    pub enable_charts: bool,
    #[serde(rename = "sideBar")]
    pub side_bar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGGridColumnDef {
    pub field: String,
    #[serde(rename = "headerName")]
    pub header_name: Option<String>,
    pub sortable: Option<bool>,
    pub filter: Option<serde_json::Value>, // boolean or string
    pub resizable: Option<bool>,
    pub width: Option<i32>,
    #[serde(rename = "minWidth")]
    pub min_width: Option<i32>,
    #[serde(rename = "maxWidth")]
    pub max_width: Option<i32>,
    #[serde(rename = "cellRenderer")]
    pub cell_renderer: Option<String>,
    #[serde(rename = "valueFormatter")]
    pub value_formatter: Option<String>,
    #[serde(rename = "aggFunc")]
    pub agg_func: Option<String>, // 'sum' | 'avg' | 'count' | 'min' | 'max'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricConfig {
    pub title: String,
    pub value: serde_json::Value,
    pub format: Option<String>, // 'number' | 'currency' | 'percentage'
    pub trend: Option<MetricTrend>,
    pub sparkline: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTrend {
    pub direction: String, // 'up' | 'down' | 'neutral'
    pub percentage: f64,
    pub period: String, // '1d' | '7d' | '30d'
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub title: String,
    pub field: String,
    #[serde(rename = "type")]
    pub filter_type: String, // 'text' | 'number' | 'date' | 'select'
    pub options: Option<Vec<serde_json::Value>>,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDashboardRequest {
    pub name: String,
    pub layout: Vec<WidgetLayout>,
    pub filters: Option<serde_json::Value>,
    pub data_source_id: Option<String>,
    pub refresh_interval: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDashboardRequest {
    pub name: Option<String>,
    pub layout: Option<Vec<WidgetLayout>>,
    pub filters: Option<serde_json::Value>,
    pub data_source_id: Option<String>,
    pub refresh_interval: Option<i32>,
}

impl DashboardConfig {
    pub fn new(id: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            layout: Vec::new(),
            filters: None,
            data_source_id: None,
            refresh_interval: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_layout(mut self, layout: Vec<WidgetLayout>) -> Self {
        self.layout = layout;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_data_source(mut self, data_source_id: String) -> Self {
        self.data_source_id = Some(data_source_id);
        self.updated_at = Utc::now();
        self
    }

    pub fn with_refresh_interval(mut self, interval: i32) -> Self {
        self.refresh_interval = Some(interval);
        self.updated_at = Utc::now();
        self
    }
}

impl WidgetLayout {
    pub fn new(id: String, widget_type: String, position: Position, config: WidgetConfig) -> Self {
        Self {
            id,
            widget_type,
            position,
            config,
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_config_creation() {
        let dashboard = DashboardConfig::new(
            "test-dashboard".to_string(),
            "Test Dashboard".to_string(),
        )
        .with_data_source("test-source".to_string())
        .with_refresh_interval(30);

        assert_eq!(dashboard.id, "test-dashboard");
        assert_eq!(dashboard.name, "Test Dashboard");
        assert_eq!(dashboard.data_source_id, Some("test-source".to_string()));
        assert_eq!(dashboard.refresh_interval, Some(30));
    }

    #[test]
    fn test_widget_layout_creation() {
        let metric_config = MetricConfig {
            title: "Total Sales".to_string(),
            value: serde_json::json!(1000000),
            format: Some("currency".to_string()),
            trend: None,
            sparkline: None,
        };

        let widget = WidgetLayout::new(
            "widget-1".to_string(),
            "metric".to_string(),
            Position::new(0, 0, 4, 2),
            WidgetConfig::Metric(metric_config),
        );

        assert_eq!(widget.id, "widget-1");
        assert_eq!(widget.widget_type, "metric");
        assert_eq!(widget.position.x, 0);
        assert_eq!(widget.position.w, 4);
    }

    #[test]
    fn test_serialization() {
        let dashboard = DashboardConfig::new(
            "test".to_string(),
            "Test".to_string(),
        );

        let json = serde_json::to_string(&dashboard).unwrap();
        let deserialized: DashboardConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(dashboard.id, deserialized.id);
        assert_eq!(dashboard.name, deserialized.name);
    }
}
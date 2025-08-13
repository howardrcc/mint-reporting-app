// API request and response types

export interface ApiResponse<T = any> {
  data?: T;
  error?: string;
  message?: string;
  code?: string;
  details?: any;
}

export interface PaginatedResponse<T> extends ApiResponse<T[]> {
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

// Data Source API
export interface CreateDataSourceRequest {
  name: string;
  type: 'file' | 'database' | 'api';
  filePath?: string;
}

export interface DataPreviewRequest {
  limit?: number;
  offset?: number;
  filters?: Record<string, any>;
}

export interface DataPreviewResponse {
  columns: string[];
  data: any[][];
  totalRows: number;
  previewRows: number;
}

// Dashboard API
export interface CreateDashboardRequest {
  name: string;
  layout: WidgetLayout[];
  filters?: any;
  dataSourceId?: string;
  refreshInterval?: number;
}

export interface UpdateDashboardRequest {
  name?: string;
  layout?: WidgetLayout[];
  filters?: any;
  dataSourceId?: string;
  refreshInterval?: number;
}

// Analytics API
export interface QueryRequest {
  sql: string;
  dataSourceId?: string;
  params?: Record<string, any>;
  cache?: boolean;
}

export interface QueryResult {
  columns: string[];
  data: any[][];
  rowCount: number;
}

export interface AggregationRequest {
  dataSourceId: string;
  operations: AggregationOperation[];
  groupBy?: string[];
  filters?: any;
  limit?: number;
}

export interface AggregationOperation {
  field: string;
  operation: 'sum' | 'avg' | 'count' | 'min' | 'max' | 'distinct_count';
  alias?: string;
}

export interface AggregationResult {
  columns: string[];
  data: any[][];
  rowCount: number;
  aggregations: AggregationSummary[];
}

export interface AggregationSummary {
  field: string;
  operation: string;
  result: any;
}

export interface ExportRequest {
  dataSourceId?: string;
  query?: string;
  format: 'csv' | 'json' | 'parquet';
  filters?: any;
  columns?: string[];
}

export interface ExportResult {
  fileUrl: string;
  fileSize: number;
  rowCount: number;
  format: string;
  expiresAt: string;
}

export interface MetricsRequest {
  dataSourceId: string;
  metrics: string[];
  timeRange?: TimeRange;
}

export interface TimeRange {
  start: string;
  end: string;
}

export interface MetricsResult {
  dataSourceId: string;
  metrics: MetricValue[];
  calculatedAt: string;
}

export interface MetricValue {
  name: string;
  value: any;
  description?: string;
  unit?: string;
}

// System API
export interface HealthResponse {
  status: string;
  timestamp: string;
  version: string;
}

export interface SystemStats {
  database: DatabaseInfo;
  memoryUsage: number;
  activeConnections: number;
  uptimeSeconds: number;
}

export interface DatabaseInfo {
  version: string;
  memoryUsage: number;
  tableCount: number;
}

// WebSocket Types
export type ClientMessage = 
  | {
      type: 'data:subscribe';
      sourceId: string;
      filters?: any;
    }
  | {
      type: 'data:unsubscribe';
      sourceId: string;
    }
  | {
      type: 'query:execute';
      sql: string;
      params?: any;
    };

export type ServerMessage = 
  | {
      type: 'data:update';
      sourceId: string;
      data: any[];
    }
  | {
      type: 'query:result';
      queryId: string;
      data: any[];
      error?: string;
    }
  | {
      type: 'system:status';
      memory: number;
      connections: number;
    }
  | {
      type: 'error';
      message: string;
      code?: string;
    };

// Re-export widget layout types
export interface WidgetLayout {
  id: string;
  type: 'ag-chart' | 'ag-grid' | 'metric' | 'filter';
  position: Position;
  config: WidgetConfig;
}

export interface Position {
  x: number;
  y: number;
  w: number;
  h: number;
}

export type WidgetConfig = AGChartConfig | AGGridConfig | MetricConfig | FilterConfig;

// These will be defined in detail in other type files
export interface AGChartConfig {
  type: string;
  data: any[];
  series: any[];
  axes?: any[];
  legend?: any;
  theme?: string;
}

export interface AGGridConfig {
  columnDefs: any[];
  rowData: any[];
  pagination: boolean;
  paginationPageSize: number;
  rowSelection: string;
  enableRangeSelection: boolean;
  enableCharts: boolean;
  sideBar: boolean;
}

export interface MetricConfig {
  title: string;
  value: any;
  format?: string;
  trend?: any;
  sparkline?: number[];
}

export interface FilterConfig {
  title: string;
  field: string;
  type: string;
  options?: any[];
  defaultValue?: any;
}
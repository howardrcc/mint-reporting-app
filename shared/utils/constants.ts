// Shared constants and configuration values

// API Configuration
export const API_CONFIG = {
  BASE_URL: process.env.NODE_ENV === 'production' 
    ? 'https://api.duckdb-dashboard.com' 
    : 'http://localhost:3000',
  TIMEOUT: 30000, // 30 seconds
  RETRY_ATTEMPTS: 3,
  RETRY_DELAY: 1000, // 1 second
} as const;

// File Upload Configuration
export const FILE_UPLOAD = {
  MAX_SIZE: 1024 * 1024 * 1024, // 1GB
  ALLOWED_TYPES: ['csv', 'json', 'parquet'] as const,
  CHUNK_SIZE: 1024 * 1024, // 1MB chunks for upload
  ALLOWED_MIME_TYPES: [
    'text/csv',
    'application/json',
    'application/octet-stream', // For parquet files
  ] as const,
} as const;

// Data Source Types
export const DATA_SOURCE_TYPES = {
  FILE: 'file',
  DATABASE: 'database',
  API: 'api',
} as const;

// Column Data Types
export const COLUMN_TYPES = {
  INTEGER: 'INTEGER',
  DOUBLE: 'DOUBLE',
  VARCHAR: 'VARCHAR',
  DATE: 'DATE',
  TIMESTAMP: 'TIMESTAMP',
  BOOLEAN: 'BOOLEAN',
} as const;

// Widget Types
export const WIDGET_TYPES = {
  CHART: 'ag-chart',
  GRID: 'ag-grid',
  METRIC: 'metric',
  FILTER: 'filter',
} as const;

// Chart Types
export const CHART_TYPES = {
  LINE: 'line',
  BAR: 'bar',
  COLUMN: 'column',
  AREA: 'area',
  SCATTER: 'scatter',
  PIE: 'pie',
  DONUT: 'donut',
  HISTOGRAM: 'histogram',
  BOX_PLOT: 'box-plot',
  CANDLESTICK: 'candlestick',
  TREEMAP: 'treemap',
  SUNBURST: 'sunburst',
  RADAR: 'radar',
  GAUGE: 'gauge',
} as const;

// Chart Themes
export const CHART_THEMES = {
  DEFAULT: 'ag-default',
  DEFAULT_DARK: 'ag-default-dark',
  MATERIAL: 'ag-material',
  PASTEL: 'ag-pastel',
  SOLAR: 'ag-solar',
  VIVID: 'ag-vivid',
  POLYCHROMA: 'ag-polychroma',
} as const;

// Grid Themes
export const GRID_THEMES = {
  ALPINE: 'ag-theme-alpine',
  ALPINE_DARK: 'ag-theme-alpine-dark',
  BALHAM: 'ag-theme-balham',
  BALHAM_DARK: 'ag-theme-balham-dark',
  MATERIAL: 'ag-theme-material',
  QUARTZ: 'ag-theme-quartz',
  QUARTZ_DARK: 'ag-theme-quartz-dark',
} as const;

// Aggregation Operations
export const AGGREGATION_OPERATIONS = {
  SUM: 'sum',
  AVG: 'avg',
  COUNT: 'count',
  MIN: 'min',
  MAX: 'max',
  DISTINCT_COUNT: 'distinct_count',
} as const;

// Filter Operators
export const FILTER_OPERATORS = {
  EQUALS: 'equals',
  NOT_EQUALS: 'not_equals',
  GREATER_THAN: 'greater_than',
  GREATER_THAN_OR_EQUAL: 'greater_than_or_equal',
  LESS_THAN: 'less_than',
  LESS_THAN_OR_EQUAL: 'less_than_or_equal',
  CONTAINS: 'contains',
  NOT_CONTAINS: 'not_contains',
  STARTS_WITH: 'starts_with',
  ENDS_WITH: 'ends_with',
  IN: 'in',
  NOT_IN: 'not_in',
  IS_NULL: 'is_null',
  IS_NOT_NULL: 'is_not_null',
  BETWEEN: 'between',
  REGEX: 'regex',
} as const;

// Filter Types
export const FILTER_TYPES = {
  TEXT: 'text',
  NUMBER: 'number',
  DATE: 'date',
  DATETIME: 'datetime',
  SELECT: 'select',
  MULTISELECT: 'multiselect',
  RANGE: 'range',
  BOOLEAN: 'boolean',
  AUTOCOMPLETE: 'autocomplete',
} as const;

// Export Formats
export const EXPORT_FORMATS = {
  CSV: 'csv',
  JSON: 'json',
  PARQUET: 'parquet',
  EXCEL: 'excel',
  PDF: 'pdf',
} as const;

// WebSocket Event Types
export const WS_EVENTS = {
  // Client to Server
  DATA_SUBSCRIBE: 'data:subscribe',
  DATA_UNSUBSCRIBE: 'data:unsubscribe',
  QUERY_EXECUTE: 'query:execute',
  
  // Server to Client
  DATA_UPDATE: 'data:update',
  QUERY_RESULT: 'query:result',
  SYSTEM_STATUS: 'system:status',
  ERROR: 'error',
} as const;

// Dashboard Layout Configuration
export const DASHBOARD_LAYOUT = {
  GRID_COLS: 12,
  GRID_ROWS: 20,
  ROW_HEIGHT: 60,
  MARGIN: [10, 10],
  CONTAINER_PADDING: [10, 10],
  MIN_WIDGET_WIDTH: 2,
  MIN_WIDGET_HEIGHT: 2,
  MAX_WIDGET_WIDTH: 12,
  MAX_WIDGET_HEIGHT: 20,
} as const;

// Performance Configuration
export const PERFORMANCE = {
  MAX_ROWS_PREVIEW: 10000,
  MAX_ROWS_EXPORT: 1000000,
  MAX_CHART_POINTS: 100000,
  MAX_GRID_ROWS: 1000000,
  VIRTUALIZATION_THRESHOLD: 1000,
  QUERY_TIMEOUT: 30000, // 30 seconds
  CACHE_TTL: 300, // 5 minutes
} as const;

// Error Codes
export const ERROR_CODES = {
  // Validation Errors
  REQUIRED: 'REQUIRED',
  INVALID_VALUE: 'INVALID_VALUE',
  INVALID_FORMAT: 'INVALID_FORMAT',
  MAX_LENGTH: 'MAX_LENGTH',
  MIN_LENGTH: 'MIN_LENGTH',
  MAX_VALUE: 'MAX_VALUE',
  MIN_VALUE: 'MIN_VALUE',
  
  // File Upload Errors
  FILE_TOO_LARGE: 'FILE_TOO_LARGE',
  INVALID_FILE_TYPE: 'INVALID_FILE_TYPE',
  UPLOAD_FAILED: 'UPLOAD_FAILED',
  
  // Database Errors
  CONNECTION_FAILED: 'CONNECTION_FAILED',
  QUERY_FAILED: 'QUERY_FAILED',
  TIMEOUT: 'TIMEOUT',
  
  // Authentication Errors
  UNAUTHORIZED: 'UNAUTHORIZED',
  FORBIDDEN: 'FORBIDDEN',
  TOKEN_EXPIRED: 'TOKEN_EXPIRED',
  
  // General Errors
  NOT_FOUND: 'NOT_FOUND',
  INTERNAL_ERROR: 'INTERNAL_ERROR',
  BAD_REQUEST: 'BAD_REQUEST',
} as const;

// Time Periods for Analytics
export const TIME_PERIODS = {
  HOUR: 'hour',
  DAY: 'day',
  WEEK: 'week',
  MONTH: 'month',
  QUARTER: 'quarter',
  YEAR: 'year',
} as const;

// Date Formats
export const DATE_FORMATS = {
  ISO: 'YYYY-MM-DDTHH:mm:ss.SSSZ',
  DATE_ONLY: 'YYYY-MM-DD',
  TIME_ONLY: 'HH:mm:ss',
  DISPLAY: 'MMM DD, YYYY',
  DISPLAY_WITH_TIME: 'MMM DD, YYYY HH:mm',
  SHORT: 'MM/DD/YYYY',
} as const;

// Color Palettes
export const COLOR_PALETTES = {
  DEFAULT: [
    '#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd',
    '#8c564b', '#e377c2', '#7f7f7f', '#bcbd22', '#17becf'
  ],
  PASTEL: [
    '#ffb3ba', '#ffdfba', '#ffffba', '#baffc9', '#bae1ff',
    '#ffb3ff', '#ffe4b3', '#b3ffb3', '#b3b3ff', '#ffb3d9'
  ],
  DARK: [
    '#2E3440', '#3B4252', '#434C5E', '#4C566A', '#5E81AC',
    '#81A1C1', '#88C0D0', '#8FBCBB', '#A3BE8C', '#EBCB8B'
  ],
  VIBRANT: [
    '#FF6B6B', '#4ECDC4', '#45B7D1', '#FFA07A', '#98D8C8',
    '#F7DC6F', '#BB8FCE', '#85C1E9', '#F8C471', '#82E0AA'
  ],
} as const;

// Default Widget Configurations
export const DEFAULT_WIDGET_CONFIG = {
  CHART: {
    type: CHART_TYPES.LINE,
    theme: CHART_THEMES.DEFAULT,
    data: [],
    series: [],
  },
  GRID: {
    pagination: true,
    paginationPageSize: 50,
    rowSelection: 'multiple',
    enableRangeSelection: true,
    enableCharts: true,
    sideBar: true,
    theme: GRID_THEMES.ALPINE,
  },
  METRIC: {
    format: 'number',
    textAlign: 'center',
  },
  FILTER: {
    type: FILTER_TYPES.TEXT,
    multiSelect: false,
    searchable: true,
  },
} as const;

// Default Dashboard Settings
export const DEFAULT_DASHBOARD = {
  REFRESH_INTERVAL: 30, // seconds
  AUTO_SAVE_INTERVAL: 10, // seconds
  GRID_SNAP: true,
  COMPACT_TYPE: 'vertical',
  ALLOW_OVERLAP: false,
} as const;

// Keyboard Shortcuts
export const KEYBOARD_SHORTCUTS = {
  SAVE: 'Ctrl+S',
  COPY: 'Ctrl+C',
  PASTE: 'Ctrl+V',
  UNDO: 'Ctrl+Z',
  REDO: 'Ctrl+Y',
  DELETE: 'Delete',
  SELECT_ALL: 'Ctrl+A',
  ZOOM_IN: 'Ctrl+=',
  ZOOM_OUT: 'Ctrl+-',
  RESET_ZOOM: 'Ctrl+0',
} as const;
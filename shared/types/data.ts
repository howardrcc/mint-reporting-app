// Data source and schema types

export interface DataSource {
  id: string;
  name: string;
  type: 'file' | 'database' | 'api';
  filePath?: string;
  schema: ColumnSchema[];
  rowCount: number;
  sizeBytes: number;
  createdAt: string;
  updatedAt: string;
}

export interface ColumnSchema {
  name: string;
  type: 'INTEGER' | 'DOUBLE' | 'VARCHAR' | 'DATE' | 'TIMESTAMP' | 'BOOLEAN';
  nullable: boolean;
  unique: boolean;
  primaryKey: boolean;
}

// File upload types
export interface FileUploadProgress {
  loaded: number;
  total: number;
  percentage: number;
}

export interface FileUploadResult {
  dataSource: DataSource;
  processingTime: number;
  errors?: string[];
  warnings?: string[];
}

// Data validation types
export interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
  statistics: DataStatistics;
}

export interface ValidationError {
  row?: number;
  column: string;
  value: any;
  message: string;
  code: string;
}

export interface ValidationWarning {
  row?: number;
  column: string;
  value: any;
  message: string;
  code: string;
}

export interface DataStatistics {
  totalRows: number;
  validRows: number;
  errorRows: number;
  columnStats: ColumnStatistics[];
}

export interface ColumnStatistics {
  name: string;
  type: string;
  nullCount: number;
  uniqueCount: number;
  minValue?: any;
  maxValue?: any;
  avgValue?: number;
  mostCommonValues?: Array<{ value: any; count: number }>;
}

// Data transformation types
export interface DataTransformation {
  id: string;
  name: string;
  type: TransformationType;
  sourceColumn: string;
  targetColumn?: string;
  parameters: Record<string, any>;
}

export type TransformationType = 
  | 'cast'           // Change data type
  | 'rename'         // Rename column
  | 'calculate'      // Create calculated column
  | 'filter'         // Filter rows
  | 'aggregate'      // Aggregate data
  | 'join'           // Join with another dataset
  | 'pivot'          // Pivot data
  | 'unpivot'        // Unpivot data
  | 'sort'           // Sort data
  | 'deduplicate'    // Remove duplicates
  | 'fill_null'      // Fill null values
  | 'extract'        // Extract part of string/date
  | 'split'          // Split column
  | 'merge';         // Merge columns

// Data query types
export interface DataQuery {
  id: string;
  name: string;
  sql: string;
  dataSourceIds: string[];
  parameters: QueryParameter[];
  createdAt: string;
  updatedAt: string;
  lastExecuted?: string;
  description?: string;
}

export interface QueryParameter {
  name: string;
  type: 'string' | 'number' | 'boolean' | 'date';
  defaultValue?: any;
  required: boolean;
  description?: string;
}

// Data filtering types
export interface DataFilter {
  column: string;
  operator: FilterOperator;
  value: any;
  values?: any[]; // For IN/NOT_IN operators
}

export type FilterOperator = 
  | 'equals'
  | 'not_equals'
  | 'greater_than'
  | 'greater_than_or_equal'
  | 'less_than'
  | 'less_than_or_equal'
  | 'contains'
  | 'not_contains'
  | 'starts_with'
  | 'ends_with'
  | 'in'
  | 'not_in'
  | 'is_null'
  | 'is_not_null'
  | 'between'
  | 'regex';

// Data sorting types
export interface DataSort {
  column: string;
  direction: 'asc' | 'desc';
}

// Data pagination types
export interface DataPagination {
  page: number;
  limit: number;
  offset: number;
}

// Advanced analytics types
export interface AnalyticsProfile {
  dataSourceId: string;
  columns: ColumnProfile[];
  correlations: CorrelationMatrix;
  outliers: OutlierAnalysis;
  patterns: PatternAnalysis;
  generatedAt: string;
}

export interface ColumnProfile {
  name: string;
  type: string;
  statistics: {
    count: number;
    nullCount: number;
    uniqueCount: number;
    min?: any;
    max?: any;
    mean?: number;
    median?: number;
    mode?: any;
    standardDeviation?: number;
    variance?: number;
    skewness?: number;
    kurtosis?: number;
  };
  distribution: {
    histogram?: Array<{ bin: string; count: number }>;
    percentiles?: Array<{ percentile: number; value: any }>;
  };
  qualityScore: number; // 0-100 score indicating data quality
}

export interface CorrelationMatrix {
  columns: string[];
  matrix: number[][];
  strongCorrelations: Array<{
    column1: string;
    column2: string;
    correlation: number;
  }>;
}

export interface OutlierAnalysis {
  method: 'iqr' | 'zscore' | 'isolation_forest';
  outliers: Array<{
    rowIndex: number;
    column: string;
    value: any;
    score: number;
  }>;
  summary: {
    totalOutliers: number;
    percentageOfData: number;
    affectedColumns: string[];
  };
}

export interface PatternAnalysis {
  trends: Array<{
    column: string;
    trendType: 'increasing' | 'decreasing' | 'stable' | 'seasonal';
    confidence: number;
    description: string;
  }>;
  anomalies: Array<{
    timestamp: string;
    column: string;
    expectedValue: any;
    actualValue: any;
    anomalyScore: number;
  }>;
  seasonality: Array<{
    column: string;
    period: string; // 'daily', 'weekly', 'monthly', 'yearly'
    strength: number;
  }>;
}
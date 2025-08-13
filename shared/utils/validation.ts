// Validation utilities for shared types

import { DataSource, ColumnSchema, DashboardConfig, WidgetLayout } from '../types';

export class ValidationError extends Error {
  constructor(
    message: string,
    public field?: string,
    public code?: string
  ) {
    super(message);
    this.name = 'ValidationError';
  }
}

export interface ValidationResult {
  isValid: boolean;
  errors: ValidationError[];
}

// Data Source Validation
export function validateDataSource(dataSource: Partial<DataSource>): ValidationResult {
  const errors: ValidationError[] = [];

  if (!dataSource.name || dataSource.name.trim().length === 0) {
    errors.push(new ValidationError('Name is required', 'name', 'REQUIRED'));
  }

  if (dataSource.name && dataSource.name.length > 255) {
    errors.push(new ValidationError('Name must be less than 255 characters', 'name', 'MAX_LENGTH'));
  }

  if (!dataSource.type || !['file', 'database', 'api'].includes(dataSource.type)) {
    errors.push(new ValidationError('Type must be one of: file, database, api', 'type', 'INVALID_VALUE'));
  }

  if (dataSource.rowCount !== undefined && dataSource.rowCount < 0) {
    errors.push(new ValidationError('Row count cannot be negative', 'rowCount', 'INVALID_VALUE'));
  }

  if (dataSource.sizeBytes !== undefined && dataSource.sizeBytes < 0) {
    errors.push(new ValidationError('Size bytes cannot be negative', 'sizeBytes', 'INVALID_VALUE'));
  }

  if (dataSource.schema) {
    dataSource.schema.forEach((column, index) => {
      const columnErrors = validateColumnSchema(column);
      columnErrors.errors.forEach(error => {
        error.field = `schema[${index}].${error.field}`;
        errors.push(error);
      });
    });
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

export function validateColumnSchema(column: Partial<ColumnSchema>): ValidationResult {
  const errors: ValidationError[] = [];

  if (!column.name || column.name.trim().length === 0) {
    errors.push(new ValidationError('Column name is required', 'name', 'REQUIRED'));
  }

  if (column.name && !/^[a-zA-Z_][a-zA-Z0-9_]*$/.test(column.name)) {
    errors.push(new ValidationError('Column name must be a valid identifier', 'name', 'INVALID_FORMAT'));
  }

  const validTypes = ['INTEGER', 'DOUBLE', 'VARCHAR', 'DATE', 'TIMESTAMP', 'BOOLEAN'];
  if (!column.type || !validTypes.includes(column.type)) {
    errors.push(new ValidationError(`Type must be one of: ${validTypes.join(', ')}`, 'type', 'INVALID_VALUE'));
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

// Dashboard Validation
export function validateDashboardConfig(dashboard: Partial<DashboardConfig>): ValidationResult {
  const errors: ValidationError[] = [];

  if (!dashboard.name || dashboard.name.trim().length === 0) {
    errors.push(new ValidationError('Dashboard name is required', 'name', 'REQUIRED'));
  }

  if (dashboard.name && dashboard.name.length > 255) {
    errors.push(new ValidationError('Dashboard name must be less than 255 characters', 'name', 'MAX_LENGTH'));
  }

  if (dashboard.refreshInterval !== undefined) {
    if (dashboard.refreshInterval < 5) {
      errors.push(new ValidationError('Refresh interval must be at least 5 seconds', 'refreshInterval', 'MIN_VALUE'));
    }
    if (dashboard.refreshInterval > 3600) {
      errors.push(new ValidationError('Refresh interval must be less than 1 hour', 'refreshInterval', 'MAX_VALUE'));
    }
  }

  if (dashboard.layout) {
    dashboard.layout.forEach((widget, index) => {
      const widgetErrors = validateWidgetLayout(widget);
      widgetErrors.errors.forEach(error => {
        error.field = `layout[${index}].${error.field}`;
        errors.push(error);
      });
    });

    // Check for overlapping widgets
    const overlappingErrors = validateWidgetOverlaps(dashboard.layout);
    errors.push(...overlappingErrors);
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

export function validateWidgetLayout(widget: Partial<WidgetLayout>): ValidationResult {
  const errors: ValidationError[] = [];

  if (!widget.id || widget.id.trim().length === 0) {
    errors.push(new ValidationError('Widget ID is required', 'id', 'REQUIRED'));
  }

  const validTypes = ['ag-chart', 'ag-grid', 'metric', 'filter'];
  if (!widget.type || !validTypes.includes(widget.type)) {
    errors.push(new ValidationError(`Widget type must be one of: ${validTypes.join(', ')}`, 'type', 'INVALID_VALUE'));
  }

  if (widget.position) {
    if (widget.position.x < 0) {
      errors.push(new ValidationError('Position X cannot be negative', 'position.x', 'INVALID_VALUE'));
    }
    if (widget.position.y < 0) {
      errors.push(new ValidationError('Position Y cannot be negative', 'position.y', 'INVALID_VALUE'));
    }
    if (widget.position.w <= 0) {
      errors.push(new ValidationError('Width must be positive', 'position.w', 'INVALID_VALUE'));
    }
    if (widget.position.h <= 0) {
      errors.push(new ValidationError('Height must be positive', 'position.h', 'INVALID_VALUE'));
    }
    if (widget.position.w > 12) {
      errors.push(new ValidationError('Width cannot exceed 12 grid units', 'position.w', 'MAX_VALUE'));
    }
  } else {
    errors.push(new ValidationError('Widget position is required', 'position', 'REQUIRED'));
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

function validateWidgetOverlaps(widgets: WidgetLayout[]): ValidationError[] {
  const errors: ValidationError[] = [];

  for (let i = 0; i < widgets.length; i++) {
    for (let j = i + 1; j < widgets.length; j++) {
      const widget1 = widgets[i];
      const widget2 = widgets[j];

      if (widgetsOverlap(widget1, widget2)) {
        errors.push(new ValidationError(
          `Widget ${widget1.id} overlaps with widget ${widget2.id}`,
          `layout`,
          'WIDGET_OVERLAP'
        ));
      }
    }
  }

  return errors;
}

function widgetsOverlap(widget1: WidgetLayout, widget2: WidgetLayout): boolean {
  const pos1 = widget1.position;
  const pos2 = widget2.position;

  return !(
    pos1.x + pos1.w <= pos2.x ||
    pos2.x + pos2.w <= pos1.x ||
    pos1.y + pos1.h <= pos2.y ||
    pos2.y + pos2.h <= pos1.y
  );
}

// File Upload Validation
export function validateFileUpload(file: File): ValidationResult {
  const errors: ValidationError[] = [];

  const maxSize = 1024 * 1024 * 1024; // 1GB
  if (file.size > maxSize) {
    errors.push(new ValidationError('File size cannot exceed 1GB', 'size', 'MAX_SIZE'));
  }

  const allowedExtensions = ['csv', 'json', 'parquet'];
  const extension = file.name.split('.').pop()?.toLowerCase();
  if (!extension || !allowedExtensions.includes(extension)) {
    errors.push(new ValidationError(
      `File type not supported. Allowed types: ${allowedExtensions.join(', ')}`,
      'type',
      'INVALID_TYPE'
    ));
  }

  if (file.name.length > 255) {
    errors.push(new ValidationError('File name must be less than 255 characters', 'name', 'MAX_LENGTH'));
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

// SQL Query Validation
export function validateSQLQuery(sql: string): ValidationResult {
  const errors: ValidationError[] = [];

  if (!sql || sql.trim().length === 0) {
    errors.push(new ValidationError('SQL query is required', 'sql', 'REQUIRED'));
  }

  // Basic SQL injection prevention
  const dangerousKeywords = ['DROP', 'DELETE', 'INSERT', 'UPDATE', 'ALTER', 'CREATE', 'TRUNCATE'];
  const upperSQL = sql.toUpperCase();
  
  for (const keyword of dangerousKeywords) {
    if (upperSQL.includes(keyword)) {
      errors.push(new ValidationError(
        `Query contains potentially dangerous keyword: ${keyword}`,
        'sql',
        'DANGEROUS_KEYWORD'
      ));
    }
  }

  // Check for balanced parentheses
  let parenthesesCount = 0;
  for (const char of sql) {
    if (char === '(') parenthesesCount++;
    if (char === ')') parenthesesCount--;
    if (parenthesesCount < 0) {
      errors.push(new ValidationError('Unbalanced parentheses in SQL query', 'sql', 'SYNTAX_ERROR'));
      break;
    }
  }
  if (parenthesesCount !== 0) {
    errors.push(new ValidationError('Unbalanced parentheses in SQL query', 'sql', 'SYNTAX_ERROR'));
  }

  return {
    isValid: errors.length === 0,
    errors
  };
}

// General utility functions
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return emailRegex.test(email);
}

export function isValidUrl(url: string): boolean {
  try {
    new URL(url);
    return true;
  } catch {
    return false;
  }
}

export function isValidUUID(uuid: string): boolean {
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  return uuidRegex.test(uuid);
}

export function sanitizeString(input: string): string {
  return input
    .replace(/[<>]/g, '') // Remove potential HTML tags
    .replace(/['"]/g, '') // Remove quotes
    .trim();
}

export function truncateString(input: string, maxLength: number): string {
  if (input.length <= maxLength) return input;
  return input.substring(0, maxLength - 3) + '...';
}
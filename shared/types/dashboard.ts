// Dashboard and widget configuration types

export interface DashboardConfig {
  id: string;
  name: string;
  layout: WidgetLayout[];
  filters?: any;
  dataSourceId?: string;
  refreshInterval?: number; // in seconds
  createdAt: string;
  updatedAt: string;
}

export interface WidgetLayout {
  id: string;
  type: WidgetType;
  position: Position;
  config: WidgetConfig;
}

export type WidgetType = 'ag-chart' | 'ag-grid' | 'metric' | 'filter';

export interface Position {
  x: number;
  y: number;
  w: number;
  h: number;
}

export type WidgetConfig = AGChartConfig | AGGridConfig | MetricConfig | FilterConfig;

// AG Charts Configuration
export interface AGChartConfig {
  type: ChartType;
  data: any[];
  series: AGChartSeries[];
  axes?: AGChartAxis[];
  legend?: AGChartLegend;
  theme?: ChartTheme;
  title?: ChartTitle;
  subtitle?: ChartSubtitle;
  animation?: ChartAnimation;
  tooltip?: ChartTooltip;
}

export type ChartType = 
  | 'line' 
  | 'bar' 
  | 'column'
  | 'area'
  | 'scatter' 
  | 'pie' 
  | 'donut'
  | 'histogram'
  | 'box-plot'
  | 'candlestick'
  | 'treemap'
  | 'sunburst'
  | 'radar'
  | 'gauge';

export interface AGChartSeries {
  type: string;
  xKey: string;
  yKey: string;
  yName?: string;
  stroke?: string;
  fill?: string;
  marker?: ChartMarker;
  lineWidth?: number;
  lineDash?: number[];
  opacity?: number;
  visible?: boolean;
}

export interface ChartMarker {
  enabled?: boolean;
  shape?: 'circle' | 'square' | 'diamond' | 'triangle' | 'cross' | 'plus';
  size?: number;
  fill?: string;
  stroke?: string;
  strokeWidth?: number;
}

export interface AGChartAxis {
  type: 'category' | 'number' | 'time' | 'log';
  position: 'bottom' | 'left' | 'top' | 'right';
  title?: AGChartAxisTitle;
  label?: AGChartAxisLabel;
  line?: AGChartAxisLine;
  tick?: AGChartAxisTick;
  gridStyle?: AGChartGridStyle[];
  min?: number;
  max?: number;
  nice?: boolean;
}

export interface AGChartAxisTitle {
  text: string;
  fontSize?: number;
  fontFamily?: string;
  fontWeight?: string;
  color?: string;
}

export interface AGChartAxisLabel {
  fontSize?: number;
  fontFamily?: string;
  color?: string;
  rotation?: number;
  format?: string;
}

export interface AGChartAxisLine {
  width?: number;
  color?: string;
}

export interface AGChartAxisTick {
  width?: number;
  size?: number;
  color?: string;
}

export interface AGChartGridStyle {
  stroke?: string;
  lineDash?: number[];
}

export interface AGChartLegend {
  enabled: boolean;
  position?: 'top' | 'bottom' | 'left' | 'right';
  spacing?: number;
  item?: LegendItem;
}

export interface LegendItem {
  paddingX?: number;
  paddingY?: number;
  marker?: LegendMarker;
  label?: LegendLabel;
}

export interface LegendMarker {
  shape?: string;
  size?: number;
  strokeWidth?: number;
  padding?: number;
}

export interface LegendLabel {
  fontSize?: number;
  fontFamily?: string;
  color?: string;
}

export interface ChartTitle {
  text: string;
  fontSize?: number;
  fontFamily?: string;
  fontWeight?: string;
  color?: string;
}

export interface ChartSubtitle {
  text: string;
  fontSize?: number;
  fontFamily?: string;
  color?: string;
}

export interface ChartAnimation {
  enabled?: boolean;
  duration?: number;
}

export interface ChartTooltip {
  enabled?: boolean;
  tracking?: boolean;
  delay?: number;
}

export type ChartTheme = 
  | 'ag-default' 
  | 'ag-default-dark' 
  | 'ag-material' 
  | 'ag-pastel'
  | 'ag-solar'
  | 'ag-vivid'
  | 'ag-polychroma';

// AG Grid Configuration
export interface AGGridConfig {
  columnDefs: AGGridColumnDef[];
  rowData: any[];
  pagination: boolean;
  paginationPageSize: number;
  rowSelection: 'single' | 'multiple';
  enableRangeSelection: boolean;
  enableCharts: boolean;
  sideBar: boolean | AGGridSideBar;
  rowHeight?: number;
  headerHeight?: number;
  defaultColDef?: AGGridColumnDef;
  rowModelType?: 'clientSide' | 'infinite' | 'serverSide' | 'viewport';
  cacheBlockSize?: number;
  maxBlocksInCache?: number;
  suppressRowClickSelection?: boolean;
  suppressCellFocus?: boolean;
  enableCellTextSelection?: boolean;
  theme?: GridTheme;
}

export type GridTheme = 
  | 'ag-theme-alpine'
  | 'ag-theme-alpine-dark'
  | 'ag-theme-balham'
  | 'ag-theme-balham-dark'
  | 'ag-theme-material'
  | 'ag-theme-quartz'
  | 'ag-theme-quartz-dark';

export interface AGGridColumnDef {
  field: string;
  headerName?: string;
  width?: number;
  minWidth?: number;
  maxWidth?: number;
  flex?: number;
  sortable?: boolean;
  filter?: boolean | string | AGGridFilter;
  resizable?: boolean;
  editable?: boolean;
  cellRenderer?: string | AGGridCellRenderer;
  cellEditor?: string | AGGridCellEditor;
  valueFormatter?: string | ((params: any) => string);
  valueGetter?: string | ((params: any) => any);
  valueSetter?: string | ((params: any) => boolean);
  aggFunc?: 'sum' | 'avg' | 'count' | 'min' | 'max' | 'first' | 'last';
  enableRowGroup?: boolean;
  enablePivot?: boolean;
  enableValue?: boolean;
  hide?: boolean;
  pinned?: 'left' | 'right';
  lockPosition?: boolean;
  suppressMenu?: boolean;
  suppressSorting?: boolean;
  suppressSizeToFit?: boolean;
  suppressAutoSize?: boolean;
  cellClass?: string | string[] | ((params: any) => string | string[]);
  cellStyle?: any | ((params: any) => any);
  headerClass?: string | string[];
  headerTooltip?: string;
  tooltip?: string | ((params: any) => string);
}

export interface AGGridFilter {
  type: 'text' | 'number' | 'date' | 'set';
  filterParams?: any;
}

export interface AGGridCellRenderer {
  component: string;
  params?: any;
}

export interface AGGridCellEditor {
  component: string;
  params?: any;
}

export interface AGGridSideBar {
  toolPanels?: Array<{
    id: string;
    labelDefault: string;
    labelKey?: string;
    iconKey: string;
    toolPanel: string;
    toolPanelParams?: any;
  }>;
  defaultToolPanel?: string;
  hiddenByDefault?: boolean;
}

// Metric Widget Configuration
export interface MetricConfig {
  title: string;
  value: any;
  format?: MetricFormat;
  trend?: MetricTrend;
  sparkline?: SparklineConfig;
  threshold?: MetricThreshold;
  icon?: string;
  color?: string;
  backgroundColor?: string;
  textAlign?: 'left' | 'center' | 'right';
}

export type MetricFormat = 
  | 'number'
  | 'currency'
  | 'percentage'
  | 'bytes'
  | 'duration'
  | 'custom';

export interface MetricTrend {
  direction: 'up' | 'down' | 'neutral';
  percentage: number;
  period: string; // '1h', '1d', '7d', '30d'
  isGood?: boolean; // Whether the trend direction is positive for this metric
}

export interface SparklineConfig {
  data: number[];
  color?: string;
  lineWidth?: number;
  showArea?: boolean;
  areaColor?: string;
  height?: number;
}

export interface MetricThreshold {
  min?: number;
  max?: number;
  target?: number;
  warningMin?: number;
  warningMax?: number;
  criticalMin?: number;
  criticalMax?: number;
}

// Filter Widget Configuration
export interface FilterConfig {
  title: string;
  field: string;
  type: FilterType;
  options?: FilterOption[];
  defaultValue?: any;
  multiSelect?: boolean;
  searchable?: boolean;
  placeholder?: string;
  width?: number;
  height?: number;
}

export type FilterType = 
  | 'text'
  | 'number'
  | 'date'
  | 'datetime'
  | 'select'
  | 'multiselect'
  | 'range'
  | 'boolean'
  | 'autocomplete';

export interface FilterOption {
  value: any;
  label: string;
  color?: string;
  icon?: string;
}

// Dashboard template types
export interface DashboardTemplate {
  id: string;
  name: string;
  description: string;
  category: string;
  tags: string[];
  thumbnail?: string;
  layout: WidgetLayout[];
  requiredDataSourceType?: string;
  sampleData?: any;
  isPublic: boolean;
  createdBy: string;
  createdAt: string;
  usageCount: number;
}

// Dashboard sharing and permissions
export interface DashboardPermissions {
  dashboardId: string;
  isPublic: boolean;
  allowedUsers: string[];
  allowedRoles: string[];
  permissions: {
    view: boolean;
    edit: boolean;
    share: boolean;
    delete: boolean;
  };
}

// Dashboard export/import types
export interface DashboardExport {
  dashboard: DashboardConfig;
  dataSourceMappings: DataSourceMapping[];
  exportedAt: string;
  version: string;
}

export interface DataSourceMapping {
  originalId: string;
  originalName: string;
  type: string;
  schema: any;
}

export interface DashboardImport {
  dashboard: DashboardConfig;
  dataSourceMappings: Record<string, string>; // originalId -> newId
  options: {
    overwriteExisting: boolean;
    preserveIds: boolean;
    updateDataSources: boolean;
  };
}
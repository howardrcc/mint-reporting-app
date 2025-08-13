# DuckDB Multi-Platform Dashboard Application Specification

## Project Overview

Create a multi-platform data dashboard application that leverages DuckDB for high-performance analytics on large datasets (millions of rows). The application will have three deployment targets:

1. **Desktop Application**: Tauri + React frontend with Rust backend
2. **Web Application**: React frontend with Node.js backend 
3. **Containerized Deployment**: Docker containers for easy deployment

## Architecture

### Core Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   React UI      │    │  Shared Rust     │    │   DuckDB        │
│   - Charts      │◄──►│  Backend Core    │◄──►│   - Analytics   │
│   - Tables      │    │  - Data Ops      │    │   - Storage     │
│   - Filters     │    │  - WebSockets    │    │   - Queries     │
└─────────────────┘    │  - File I/O      │    └─────────────────┘
                       └──────────────────┘
                              ▲
                              │
                    ┌─────────┴─────────┐
                    │                   │
            ┌───────▼────────┐ ┌────────▼────────┐
            │  Tauri Wrapper │ │  Web Server     │
            │  (Desktop)     │ │  (Axum/Warp)    │
            └────────────────┘ └─────────────────┘
```

### Technology Stack

**Frontend (React)**
- React 18.3+ with TypeScript 5.3+
- Vite 5.0+ for build tooling
- AG Grid Community 32.0+ for data tables
- AG Charts Community 10.0+ for visualizations
- Tailwind CSS 3.4+ for styling
- React Query (TanStack Query) 5.0+ for data fetching
- React Hook Form 7.48+ for form management
- Framer Motion 11.0+ for animations
- WebSocket client for real-time updates

**Backend (Shared Rust)**
- Rust 1.75+ with DuckDB 0.9+ Rust bindings
- Tauri 2.0+ for desktop integration
- Axum 0.7+ for web server (web deployment)
- Serde 1.0+ for JSON serialization
- Tokio 1.35+ for async operations
- Tower 0.4+ middleware for CORS, logging, compression
- tokio-tungstenite 0.21+ for WebSocket support
- uuid 1.6+ for unique identifiers
- chrono 0.4+ for date/time handling

**Database**
- DuckDB with Rust bindings for both platforms
- Support for Parquet, CSV, JSON ingestion
- In-memory analytics and caching
- Shared database schema and migrations

**Development & Deployment**
- Docker 24.0+ for web deployment
- Node.js 20+ for frontend tooling
- Cross-platform desktop builds (Windows, macOS, Linux)
- VS Code Dev Containers with Rust 1.75+ toolchain
- GitHub Actions for CI/CD
- ESLint 8.0+ and Prettier 3.0+ for code quality

## Project Structure

```
duckdb-dashboard/
├── README.md
├── spec.md
├── .gitignore
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── docker/
│   ├── docker-compose.yml
│   ├── web.Dockerfile
│   └── nginx.Dockerfile
├── shared/
│   ├── types/
│   │   ├── api.ts
│   │   ├── data.ts
│   │   └── dashboard.ts
│   └── utils/
│       ├── validation.ts
│       └── constants.ts
├── backend/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── lib.rs
│       ├── main.rs              # Web server entry point
│       ├── database/
│       │   ├── mod.rs
│       │   ├── connection.rs
│       │   ├── queries.rs
│       │   └── migrations.rs
│       ├── handlers/
│       │   ├── mod.rs
│       │   ├── data.rs
│       │   ├── files.rs
│       │   ├── dashboard.rs
│       │   └── websocket.rs
│       ├── services/
│       │   ├── mod.rs
│       │   ├── duckdb.rs
│       │   ├── file_processor.rs
│       │   └── analytics.rs
│       ├── middleware/
│       │   ├── mod.rs
│       │   ├── cors.rs
│       │   ├── logging.rs
│       │   └── validation.rs
│       ├── models/
│       │   ├── mod.rs
│       │   ├── data_source.rs
│       │   ├── dashboard.rs
│       │   └── query.rs
│       └── utils/
│           ├── mod.rs
│           ├── error.rs
│           └── config.rs
├── desktop/
│   ├── src-tauri/
│   │   ├── Cargo.toml
│   │   ├── tauri.conf.json
│   │   ├── capabilities/          # Tauri 2.0 capability system
│   │   │   ├── default.json
│   │   │   └── file-access.json
│   │   └── src/
│   │       ├── main.rs          # Tauri-specific wrapper
│   │       └── lib.rs
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   └── src/
│       ├── App.tsx
│       ├── main.tsx
│       ├── components/
│       ├── hooks/
│       ├── services/
│       └── types/
├── web-frontend/
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   └── src/
│       ├── App.tsx
│       ├── main.tsx
│       ├── components/
│       │   ├── Dashboard/
│       │   │   ├── Dashboard.tsx
│       │   │   ├── AGChartsPanel.tsx
│       │   │   ├── AGDataGrid.tsx
│       │   │   ├── FilterPanel.tsx
│       │   │   └── MetricsCards.tsx
│       │   ├── DataImport/
│       │   │   ├── FileUpload.tsx
│       │   │   ├── DataPreview.tsx
│       │   │   └── SchemaMapper.tsx
│       │   └── Common/
│       │       ├── Layout.tsx
│       │       ├── Navbar.tsx
│       │       └── LoadingSpinner.tsx
│       ├── hooks/
│       │   ├── useWebSocket.ts
│       │   ├── useDataQuery.ts
│       │   ├── useAGGrid.ts
│       │   └── useAGCharts.ts
│       ├── services/
│       │   ├── api.ts
│       │   ├── websocket.ts
│       │   └── dataProcessing.ts
│       ├── types/
│       └── utils/
└── web-deployment/
    ├── nginx.conf
    └── docker-compose.override.yml
```

## Architecture Benefits

### Shared Rust Backend Advantages
- **Code Reuse**: 90%+ backend logic shared between platforms
- **Performance**: Rust's zero-cost abstractions and memory safety
- **Consistency**: Identical API behavior across desktop and web
- **Type Safety**: Strong typing prevents runtime errors
- **DuckDB Integration**: Native Rust bindings for optimal performance
- **Maintenance**: Single codebase for all backend logic

### Platform-Specific Adaptations
- **Desktop (Tauri)**: File system access, native OS integration
- **Web (Axum Server)**: HTTP endpoints, CORS, rate limiting
- **Shared Core**: Database operations, business logic, data processing

### 1. Data Import & Management
- **File Upload**: Support CSV, Parquet, JSON files up to 1GB
- **Data Preview**: Show first 1000 rows with column types
- **Schema Detection**: Auto-detect column types and suggest transformations
- **Data Validation**: Check for missing values, duplicates, data quality issues
- **Multiple Sources**: Connect to databases, APIs, cloud storage

### 2. Dashboard Components
- **AG Charts**: Line, bar, scatter, pie, donut, area charts with advanced features
- **AG Grid**: Enterprise-grade data tables with millions of rows support
  - Row virtualization for performance
  - Column sorting, filtering, grouping
  - Real-time data updates
  - Export capabilities (CSV, Excel)
- **Metrics Cards**: KPI summary cards with trend indicators and sparklines
- **Filter Panel**: Dynamic filters with AG Grid's advanced filtering
- **Export Options**: PNG, PDF, Excel export via AG Grid/Charts APIs

### 3. Analytics Engine
- **SQL Query Interface**: Write custom DuckDB queries
- **Aggregations**: Group by, sum, count, average operations
- **Time Series**: Date-based analysis and trending
- **Statistical Functions**: Percentiles, standard deviation, correlation
- **Real-time Updates**: WebSocket-based live data updates

### 4. Performance Optimization
- **AG Grid Virtualization**: Built-in row virtualization for millions of rows
- **AG Charts Performance**: Hardware-accelerated rendering with WebGL
- **Query Caching**: Cache frequent queries and aggregations
- **Incremental Loading**: Load data in chunks for large datasets
- **Background Processing**: Async data processing and imports
- **Memory Management**: Efficient memory usage with AG Grid's data management

## API Specification

### REST Endpoints

```typescript
// Data Management
POST   /api/data/upload          // Upload data files
GET    /api/data/sources         // List available data sources
DELETE /api/data/sources/:id     // Delete data source
GET    /api/data/schema/:id      // Get data schema
POST   /api/data/preview/:id     // Preview data with filters

// Dashboard
GET    /api/dashboard/configs    // Get saved dashboard configs
POST   /api/dashboard/configs    // Save dashboard configuration
PUT    /api/dashboard/configs/:id // Update dashboard
DELETE /api/dashboard/configs/:id // Delete dashboard

// Analytics
POST   /api/analytics/query      // Execute custom SQL query
POST   /api/analytics/aggregate  // Run aggregation operations
GET    /api/analytics/metrics/:id // Get predefined metrics
POST   /api/analytics/export     // Export data/charts

// System
GET    /api/system/health        // Health check
GET    /api/system/stats         // Database statistics
POST   /api/system/optimize      // Optimize database
```

### WebSocket Events

```typescript
// Client to Server
interface ClientEvents {
  'data:subscribe': { sourceId: string, filters?: object }
  'data:unsubscribe': { sourceId: string }
  'query:execute': { sql: string, params?: object }
}

// Server to Client  
interface ServerEvents {
  'data:update': { sourceId: string, data: object[] }
  'query:result': { queryId: string, data: object[], error?: string }
  'system:status': { memory: number, connections: number }
}
```

### Frontend Package.json Dependencies

```json
{
  "dependencies": {
    "react": "^18.3.0",
    "react-dom": "^18.3.0",
    "ag-grid-community": "^32.0.0",
    "ag-grid-react": "^32.0.0",
    "ag-charts-community": "^10.0.0",
    "ag-charts-react": "^10.0.0",
    "@tanstack/react-query": "^5.0.0",
    "react-hook-form": "^7.48.0",
    "tailwindcss": "^3.4.0",
    "framer-motion": "^11.0.0",
    "uuid": "^9.0.0",
    "date-fns": "^3.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.3.0",
    "@types/react-dom": "^18.3.0",
    "@types/uuid": "^9.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "@vitejs/plugin-react": "^4.2.0",
    "eslint": "^8.0.0",
    "prettier": "^3.0.0",
    "@tauri-apps/cli": "^2.0.0"
  }
}
```

### Backend Cargo.toml Dependencies

```toml
[dependencies]
duckdb = "0.9"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "compression"] }
tokio-tungstenite = "0.21"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"
clap = { version = "4.0", features = ["derive"] }

[target.'cfg(desktop)'.dependencies]
tauri = { version = "2.0", features = ["api-all"] }
```

## Data Models

### TypeScript Interfaces

```typescript
interface DataSource {
  id: string
  name: string
  type: 'file' | 'database' | 'api'
  schema: ColumnSchema[]
  rowCount: number
  size: number
  createdAt: Date
  updatedAt: Date
}

interface ColumnSchema {
  name: string
  type: 'INTEGER' | 'DOUBLE' | 'VARCHAR' | 'DATE' | 'TIMESTAMP' | 'BOOLEAN'
  nullable: boolean
  unique: boolean
  primaryKey: boolean
}

interface DashboardConfig {
  id: string
  name: string
  layout: WidgetLayout[]
  filters: FilterConfig[]
  dataSource: string
  refreshInterval?: number
}

interface WidgetLayout {
  id: string
  type: 'ag-chart' | 'ag-grid' | 'metric' | 'filter'
  position: { x: number, y: number, w: number, h: number }
  config: AGChartConfig | AGGridConfig | MetricConfig
}

interface AGChartConfig {
  type: 'line' | 'bar' | 'scatter' | 'pie' | 'donut' | 'area' | 'column'
  data: any[]
  series: AGChartSeries[]
  axes?: AGChartAxis[]
  legend?: AGChartLegend
  theme?: 'ag-default' | 'ag-default-dark' | 'ag-material' | 'ag-pastel'
}

interface AGGridConfig {
  columnDefs: AGGridColumnDef[]
  rowData: any[]
  pagination: boolean
  paginationPageSize: number
  rowSelection: 'single' | 'multiple'
  enableRangeSelection: boolean
  enableCharts: boolean
  sideBar: boolean
}

interface AGChartSeries {
  type: string
  xKey: string
  yKey: string
  yName?: string
  stroke?: string
  fill?: string
  marker?: object
}

interface AGGridColumnDef {
  field: string
  headerName?: string
  sortable?: boolean
  filter?: boolean | string
  resizable?: boolean
  width?: number
  minWidth?: number
  maxWidth?: number
  cellRenderer?: string
  valueFormatter?: string
  aggFunc?: 'sum' | 'avg' | 'count' | 'min' | 'max'
}
```

## Development Setup

### Prerequisites
- Node.js 20+
- Rust 1.75+
- Tauri CLI 2.0+
- Docker 24.0+ & Docker Compose v2
- VS Code (recommended)

### Getting Started

```bash
# Clone repository
git clone <repository-url>
cd duckdb-dashboard

# Install Tauri CLI 2.0
npm install -g @tauri-apps/cli@next

# Setup development container (recommended)
code .
# VS Code will prompt to reopen in container

# Or setup locally
cd web-frontend && npm install
cd ../desktop && npm install
cd ../backend && cargo build
```

### Development Commands

```bash
# Shared backend development
cd backend
cargo run                   # Run web server locally
cargo test                  # Run backend tests
cargo check                 # Check for compilation errors

# Web development
npm run dev:web             # Start web frontend + Rust backend
npm run dev:frontend        # Frontend only (assumes backend running)
npm run backend:web         # Run Rust backend in web mode

# Desktop development  
npm run dev:desktop         # Start Tauri development (uses shared backend)
npm run build:desktop       # Build desktop application

# Docker development (web only)
docker-compose up -dev      # Web stack with hot reload

# Testing
npm run test                # Run all tests
npm run test:frontend       # Frontend tests only
cargo test                  # Backend tests (from backend/)
npm run test:e2e            # End-to-end tests

# Building
npm run build:web           # Build web application + Rust backend
npm run build:desktop       # Build desktop application  
npm run build:docker        # Build Docker images for web
```

## Deployment Configurations

### Docker Compose (docker/docker-compose.yml)

```yaml
services:
  backend:
    build:
      context: ../backend
      dockerfile: ../docker/web.Dockerfile
    environment:
      - RUST_ENV=production
      - DUCKDB_PATH=/data/app.db
      - HOST=0.0.0.0
      - PORT=3000
    volumes:
      - ./data:/data
    ports:
      - "3000:3000"

  frontend:
    build:
      context: ../web-frontend
      dockerfile: ../docker/nginx.Dockerfile
    ports:
      - "80:80"
    depends_on:
      - backend
    environment:
      - VITE_API_URL=http://backend:3000
```

### Dev Container (.devcontainer/devcontainer.json)

```json
{
  "name": "DuckDB Dashboard Dev",
  "build": { "dockerfile": "Dockerfile" },
  "features": {
    "ghcr.io/devcontainers/features/node:1": { "version": "20" },
    "ghcr.io/devcontainers/features/rust:1": { "version": "1.75" },
    "ghcr.io/devcontainers/features/docker-in-docker:2": {}
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer",
        "tauri-apps.tauri-vscode",
        "bradlc.vscode-tailwindcss",
        "esbenp.prettier-vscode",
        "ms-vscode.vscode-typescript-next"
      ]
    }
  },
  "forwardPorts": [3000, 5173, 1420],
  "postCreateCommand": "npm install && cd backend && cargo build"
}
```

## Performance Requirements

### Targets
- **Data Loading**: Handle 10M+ rows within 30 seconds
- **AG Grid Performance**: Sub-second rendering of 1M+ rows with virtualization
- **AG Charts Rendering**: 60fps animations with 100K+ data points
- **Query Response**: Sub-second response for aggregations on 1M+ rows  
- **Memory Usage**: <2GB RAM for 100M row datasets
- **Startup Time**: <5 seconds for desktop app launch (Tauri 2.0 improvements)

### Optimization Strategies
- Use DuckDB's columnar storage for analytics
- Leverage AG Grid's enterprise-grade virtualization
- Implement AG Charts' WebGL rendering for large datasets
- Use query result caching with Rust's memory efficiency
- Implement progressive data loading with AG Grid's infinite scrolling
- Use database indexes for common query patterns
- Leverage Tauri 2.0's improved startup performance

## Testing Strategy

### Unit Tests
- Database connection and query functions
- Data validation and transformation utilities  
- Chart component rendering with mock data
- API endpoint request/response handling

### Integration Tests
- File upload and processing pipeline
- Dashboard configuration save/load
- WebSocket real-time data flow
- Cross-platform API compatibility

### Performance Tests
- Large dataset import benchmarks
- Query execution time measurements
- Memory usage profiling
- Concurrent user simulation

### End-to-End Tests
- Complete user workflows (import → visualize → export)
- Cross-browser compatibility (web version)
- Desktop app installation and updates

## Security Considerations

### Data Protection
- Input validation for all file uploads
- SQL injection prevention in query builder
- File size and type restrictions
- Secure file storage and cleanup

### Authentication (Future)
- JWT-based authentication system
- Role-based access control
- Session management
- API rate limiting

## Future Enhancements

### Phase 2 Features
- Multi-user support with authentication
- Scheduled data imports and updates
- Advanced statistical functions
- Custom plugin system
- Cloud deployment options (AWS, GCP, Azure)

### Phase 3 Features  
- Machine learning integration
- Advanced data pipelines
- Collaboration features
- Mobile responsive design
- Enterprise SSO integration

## Documentation Requirements

### User Documentation
- Installation guides for each platform
- Data import tutorials
- Dashboard creation guides
- SQL query examples
- Troubleshooting guide

### Developer Documentation
- API reference documentation
- Database schema documentation
- Build and deployment guides
- Contributing guidelines
- Architecture decision records

## Success Metrics

### Technical Metrics
- Query performance benchmarks
- Memory usage efficiency
- Application startup time
- Error rates and reliability

### User Experience Metrics
- Time to first visualization
- Feature adoption rates
- User workflow completion rates
- Support ticket volume

This specification provides a comprehensive foundation for building a high-performance, multi-platform data dashboard application using DuckDB, React, and Tauri.
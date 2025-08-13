# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a multi-platform DuckDB data dashboard application with three deployment targets:
1. **Desktop Application**: Tauri + React frontend with Rust backend
2. **Web Application**: React frontend with Node.js backend  
3. **Containerized Deployment**: Docker containers for easy deployment

## Architecture

### Core Technology Stack
- **Frontend**: React 18.3+ with TypeScript, Vite, AG Grid/Charts, Tailwind CSS
- **Backend**: Shared Rust codebase using DuckDB bindings, Tauri 2.0, Axum web server
- **Database**: DuckDB with support for Parquet, CSV, JSON ingestion
- **Development**: Docker, VS Code Dev Containers, GitHub Actions CI/CD

### Project Structure (Expected)
```
/
├── backend/           # Shared Rust backend with DuckDB integration
├── desktop/           # Tauri desktop app wrapper
├── web-frontend/      # React web frontend
├── web-deployment/    # Docker deployment configs
├── shared/           # Shared TypeScript types and utils
└── docker/           # Docker configurations
```

## Development Commands

### Backend (Rust)
```bash
cd backend
cargo run                   # Run web server locally
cargo test                  # Run backend tests  
cargo check                 # Check compilation
```

### Frontend Development
```bash
npm run dev:web             # Start web frontend + Rust backend
npm run dev:frontend        # Frontend only (backend must be running)
npm run backend:web         # Run Rust backend in web mode
npm run dev:desktop         # Start Tauri development
```

### Building
```bash
npm run build:web           # Build web application + Rust backend
npm run build:desktop       # Build desktop application
npm run build:docker        # Build Docker images for web
```

### Testing
```bash
npm run test                # Run all tests
npm run test:frontend       # Frontend tests only
cargo test                  # Backend tests (from backend/)
npm run test:e2e            # End-to-end tests
```

### Docker Development
```bash
docker-compose up -dev      # Web stack with hot reload
```

## Key Components

### Backend Services
- **DuckDB Service**: High-performance analytics engine handling millions of rows
- **File Processor**: Handles CSV, Parquet, JSON ingestion up to 1GB
- **WebSocket Handler**: Real-time data updates and query results
- **Analytics Engine**: SQL query interface with caching

### Frontend Components
- **AG Charts**: Line, bar, scatter, pie charts with hardware acceleration
- **AG Grid**: Enterprise-grade data tables with row virtualization
- **Dashboard**: Configurable layouts with drag-drop widgets
- **Data Import**: File upload with schema detection and validation

### API Endpoints
- `POST /api/data/upload` - Upload data files
- `GET /api/data/sources` - List available data sources
- `POST /api/analytics/query` - Execute custom SQL queries
- `GET /api/dashboard/configs` - Get saved dashboard configurations

## Performance Targets
- Handle 10M+ rows within 30 seconds
- Sub-second AG Grid rendering of 1M+ rows with virtualization
- Query response under 1 second for aggregations on 1M+ rows
- Memory usage under 2GB for 100M row datasets

## Development Notes
- This project uses AG Grid Community and AG Charts Community for data visualization
- The backend is designed to be shared between Tauri desktop and web deployments
- WebSocket connections provide real-time data updates
- DuckDB provides columnar storage optimized for analytics workloads
- File uploads support automatic schema detection and data quality checks
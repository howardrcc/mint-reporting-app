# DuckDB Multi-Platform Dashboard Application

A high-performance data dashboard application that leverages DuckDB for analytics on large datasets with three deployment targets: Desktop (Tauri), Web (React + Rust), and Docker containers.

## Features

- **Multi-Platform**: Desktop app with Tauri 2.0, web application, and containerized deployment
- **High Performance**: Handle 10M+ rows with DuckDB columnar storage
- **Interactive Visualizations**: AG Grid and AG Charts for enterprise-grade data tables and charts
- **Real-time Updates**: WebSocket-based live data updates
- **File Support**: CSV, Parquet, JSON ingestion up to 1GB
- **Modern Stack**: React 18.3+, TypeScript 5.3+, Rust 1.75+, Vite 5.0+

## Quick Start

### Prerequisites

- Node.js 20+
- Rust 1.75+
- Tauri CLI 2.0+
- Docker 24.0+ & Docker Compose v2

### Development Setup

```bash
# Install dependencies
npm install

# Install Tauri CLI
npm install -g @tauri-apps/cli@next

# Start web development
npm run dev:web

# Start desktop development
npm run dev:desktop

# Start with Docker
docker-compose up -dev
```

## Architecture

### Technology Stack

- **Frontend**: React 18.3+ with TypeScript, AG Grid/Charts, Tailwind CSS
- **Backend**: Shared Rust codebase with DuckDB, Axum web server, WebSockets
- **Desktop**: Tauri 2.0 wrapper with native OS integration
- **Database**: DuckDB with columnar analytics engine

### Project Structure

```
├── shared/           # Shared TypeScript types and utilities
├── backend/          # Shared Rust backend with DuckDB integration
├── desktop/          # Tauri desktop application
├── web-frontend/     # React web frontend
├── web-deployment/   # Docker deployment configurations
└── docker/           # Container build files
```

## Development Commands

```bash
# Backend
npm run backend:web         # Run Rust backend in web mode
cargo test                  # Run backend tests (from backend/)

# Frontend
npm run dev:web             # Web frontend + Rust backend
npm run dev:frontend        # Frontend only
npm run dev:desktop         # Tauri desktop development

# Building
npm run build:web           # Build web application
npm run build:desktop       # Build desktop application
npm run build:docker        # Build Docker images

# Testing
npm run test                # Run all tests
npm run test:frontend       # Frontend tests only
npm run test:e2e            # End-to-end tests
```

## Performance Targets

- **Data Loading**: 10M+ rows within 30 seconds
- **Grid Rendering**: Sub-second rendering of 1M+ rows with virtualization
- **Query Response**: Under 1 second for aggregations on 1M+ rows
- **Memory Usage**: Under 2GB RAM for 100M row datasets

## API Endpoints

- `POST /api/data/upload` - Upload data files
- `GET /api/data/sources` - List available data sources
- `POST /api/analytics/query` - Execute custom SQL queries
- `GET /api/dashboard/configs` - Dashboard configurations

## Deployment

### Web Deployment (Docker)
```bash
docker-compose up
```

### Desktop Distribution
```bash
npm run build:desktop
# Installers generated in desktop/src-tauri/target/release/bundle/
```

## License

MIT License - see LICENSE file for details
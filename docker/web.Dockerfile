# Multi-stage build for Rust backend
FROM rust:1.75-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy Cargo files
COPY backend/Cargo.toml backend/Cargo.lock ./

# Copy source code
COPY backend/src ./src
COPY shared ./shared

# Build the application
RUN cargo build --release --bin web-server

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Create directories
RUN mkdir -p /app /data /uploads && \
    chown -R appuser:appuser /app /data /uploads

# Copy the binary
COPY --from=builder /app/target/release/web-server /app/

# Change to app user
USER appuser

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the application
CMD ["./web-server", "--host", "0.0.0.0", "--port", "3000", "--database-path", "/data/app.db"]
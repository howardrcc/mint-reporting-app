FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Install system dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    ca-certificates \
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install additional tools
RUN curl -fsSL https://get.docker.com | sh \
    && usermod -aG docker vscode

# Set up workspace
WORKDIR /workspaces/duckdb-dashboard

# Create volume mount points
RUN mkdir -p /workspaces/duckdb-dashboard/node_modules \
    && mkdir -p /workspaces/duckdb-dashboard/backend/target \
    && chown -R vscode:vscode /workspaces/duckdb-dashboard
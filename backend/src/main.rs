use std::net::SocketAddr;

use clap::Parser;
use duckdb_dashboard_backend::{create_app, database, utils::config::Config};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "duckdb-dashboard-backend")]
#[command(about = "DuckDB Dashboard Backend Server")]
struct Cli {
    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value = "3000")]
    port: u16,

    /// Database file path
    #[arg(long, default_value = "dashboard.db")]
    database_path: String,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(&cli.log_level)
        .init();

    info!("Starting DuckDB Dashboard Backend Server");

    // Initialize configuration
    let config = Config::new(cli.database_path.clone(), cli.host.clone(), cli.port);
    
    // Initialize database
    database::init(&cli.database_path).await?;
    info!("Database initialized at: {}", cli.database_path);

    // Create the application
    let app = create_app();

    // Bind to address
    let addr = SocketAddr::new(
        cli.host.parse().unwrap_or_else(|_| {
            warn!("Invalid host address, falling back to 127.0.0.1");
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        }),
        cli.port,
    );

    info!("Server listening on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
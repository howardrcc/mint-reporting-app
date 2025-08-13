// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tracing::{info, error};

use duckdb_dashboard_backend::{
    create_app, 
    database::{DatabasePool, init as db_init},
    handlers::data::AppState,
    services::file_processor::FileProcessor,
};

// Application state for Tauri
struct TauriAppState {
    backend_state: AppState,
    server_handle: Option<tokio::task::JoinHandle<()>>,
}

#[tauri::command]
async fn get_health() -> Result<String, String> {
    Ok("Desktop app is running".to_string())
}

#[tauri::command]
async fn get_data_sources(state: State<'_, Arc<Mutex<TauriAppState>>>) -> Result<Vec<String>, String> {
    // This would integrate with the backend to get actual data sources
    Ok(vec!["Sample Data 1".to_string(), "Sample Data 2".to_string()])
}

#[tauri::command]
async fn upload_file(
    path: String, 
    state: State<'_, Arc<Mutex<TauriAppState>>>
) -> Result<String, String> {
    info!("Uploading file: {}", path);
    
    // Read file and process it using the backend
    let file_data = std::fs::read(&path).map_err(|e| e.to_string())?;
    let file_name = std::path::Path::new(&path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    let state_guard = state.lock().await;
    match state_guard.backend_state.file_processor.process_file(file_name, file_data).await {
        Ok(data_source) => Ok(format!("Successfully processed: {}", data_source.name)),
        Err(e) => Err(format!("Failed to process file: {}", e)),
    }
}

#[tauri::command]
async fn execute_query(
    sql: String,
    state: State<'_, Arc<Mutex<TauriAppState>>>
) -> Result<String, String> {
    info!("Executing query: {}", sql);
    
    // This would execute the query using the backend services
    Ok(format!("Query executed: {}", sql))
}

async fn start_backend_server(app_state: AppState) -> anyhow::Result<tokio::task::JoinHandle<()>> {
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    
    info!("Backend server starting on http://127.0.0.1:3001");
    
    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("Backend server error: {}", e);
        }
    });
    
    Ok(handle)
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting DuckDB Dashboard Desktop Application");
    
    // Get application data directory
    let app_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("duckdb-dashboard");
    
    std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");
    
    let db_path = app_dir.join("desktop.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    
    // Initialize database
    if let Err(e) = db_init(&db_path_str).await {
        error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }
    
    // Create backend state
    let db_pool = DatabasePool::new(&db_path_str)
        .expect("Failed to create database pool");
    let file_processor = FileProcessor::new(db_pool.clone());
    let backend_state = AppState {
        db_pool,
        file_processor,
    };
    
    // Start backend server
    let server_handle = start_backend_server(backend_state.clone()).await
        .expect("Failed to start backend server");
    
    let tauri_state = Arc::new(Mutex::new(TauriAppState {
        backend_state,
        server_handle: Some(server_handle),
    }));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init())
        .manage(tauri_state)
        .invoke_handler(tauri::generate_handler![
            get_health,
            get_data_sources,
            upload_file,
            execute_query
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
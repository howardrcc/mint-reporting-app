// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use tracing::{info, error};

// Simplified application state for desktop
struct TauriAppState {
    db_path: String,
}

#[tauri::command]
async fn get_health() -> Result<String, String> {
    Ok("Desktop app is running".to_string())
}

#[tauri::command]
async fn get_data_sources(_state: State<'_, Arc<Mutex<TauriAppState>>>) -> Result<Vec<String>, String> {
    // Return sample data sources for now
    Ok(vec![
        "Sample CSV Data".to_string(), 
        "Sample JSON Data".to_string(),
        "Sample Parquet Data".to_string()
    ])
}

#[tauri::command]
async fn upload_file(
    path: String, 
    _state: State<'_, Arc<Mutex<TauriAppState>>>
) -> Result<String, String> {
    info!("File selected for upload: {}", path);
    
    // For now, just validate the file exists
    if std::path::Path::new(&path).exists() {
        Ok(format!("File ready for processing: {}", path))
    } else {
        Err("File not found".to_string())
    }
}

#[tauri::command]
async fn execute_query(
    sql: String,
    _state: State<'_, Arc<Mutex<TauriAppState>>>
) -> Result<String, String> {
    info!("Query to execute: {}", sql);
    
    // Basic SQL validation
    if sql.trim().is_empty() {
        return Err("Query cannot be empty".to_string());
    }
    
    Ok(format!("Query would be executed: {}", sql))
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
    
    let tauri_state = Arc::new(Mutex::new(TauriAppState {
        db_path: db_path_str,
    }));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
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
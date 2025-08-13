// Desktop-specific library functions and utilities

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DesktopConfig {
    pub database_path: String,
    pub auto_start: bool,
    pub check_updates: bool,
    pub theme: String,
    pub window_state: WindowState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub maximized: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            database_path: "desktop.db".to_string(),
            auto_start: false,
            check_updates: true,
            theme: "light".to_string(),
            window_state: WindowState {
                width: 1200,
                height: 800,
                x: 100,
                y: 100,
                maximized: false,
            },
        }
    }
}

impl DesktopConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
            .join("duckdb-dashboard");
        
        std::fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.json");
        
        if config_path.exists() {
            let config_str = std::fs::read_to_string(config_path)?;
            let config: Self = serde_json::from_str(&config_str)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> anyhow::Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
            .join("duckdb-dashboard");
        
        std::fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.json");
        let config_str = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, config_str)?;
        
        Ok(())
    }
}

pub mod desktop_api {
    use super::*;
    use tauri::{command, State};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    #[command]
    pub async fn get_desktop_config() -> Result<DesktopConfig, String> {
        DesktopConfig::load().map_err(|e| e.to_string())
    }
    
    #[command]
    pub async fn save_desktop_config(config: DesktopConfig) -> Result<(), String> {
        config.save().map_err(|e| e.to_string())
    }
    
    #[command]
    pub async fn get_app_data_dir() -> Result<String, String> {
        dirs::data_dir()
            .ok_or_else(|| "Failed to get data directory".to_string())
            .map(|path| path.join("duckdb-dashboard").to_string_lossy().to_string())
    }
    
    #[command]
    pub async fn open_file_dialog() -> Result<Option<String>, String> {
        // This would use the tauri dialog plugin to open a file picker
        // For now, return a placeholder
        Ok(Some("/path/to/selected/file.csv".to_string()))
    }
}
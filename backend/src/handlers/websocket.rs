use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::{handlers::data::AppState, utils::error::AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "data:subscribe")]
    DataSubscribe {
        #[serde(rename = "sourceId")]
        source_id: String,
        filters: Option<serde_json::Value>,
    },
    #[serde(rename = "data:unsubscribe")]
    DataUnsubscribe {
        #[serde(rename = "sourceId")]
        source_id: String,
    },
    #[serde(rename = "query:execute")]
    QueryExecute {
        sql: String,
        params: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "data:update")]
    DataUpdate {
        #[serde(rename = "sourceId")]
        source_id: String,
        data: Vec<serde_json::Value>,
    },
    #[serde(rename = "query:result")]
    QueryResult {
        #[serde(rename = "queryId")]
        query_id: String,
        data: Vec<serde_json::Value>,
        error: Option<String>,
    },
    #[serde(rename = "system:status")]
    SystemStatus {
        memory: i64,
        connections: i32,
    },
    #[serde(rename = "error")]
    Error {
        message: String,
        code: Option<String>,
    },
}

pub type MessageSender = broadcast::Sender<ServerMessage>;
pub type MessageReceiver = broadcast::Receiver<ServerMessage>;

/// WebSocket handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    info!("New WebSocket connection established");
    
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = broadcast::channel::<ServerMessage>(100);
    
    // Track active subscriptions for this connection
    let mut subscriptions: HashMap<String, bool> = HashMap::new();
    
    // Send initial system status
    let system_status = ServerMessage::SystemStatus {
        memory: 0, // TODO: Get actual memory usage
        connections: 1, // TODO: Track actual connection count
    };
    
    if let Err(e) = tx.send(system_status) {
        error!("Failed to send initial system status: {}", e);
    }
    
    // Handle incoming messages
    let tx_clone = tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!("Received WebSocket message: {}", text);
                    
                    if let Err(e) = handle_client_message(&text, &state, &tx_clone, &mut subscriptions).await {
                        error!("Error handling client message: {}", e);
                        
                        let error_msg = ServerMessage::Error {
                            message: e.to_string(),
                            code: Some("MESSAGE_HANDLER_ERROR".to_string()),
                        };
                        
                        if let Err(send_err) = tx_clone.send(error_msg) {
                            error!("Failed to send error message: {}", send_err);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by client");
                    break;
                }
                Ok(_) => {
                    // Ignore other message types (Binary, Ping, Pong)
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    });
    
    // Handle outgoing messages
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = match serde_json::to_string(&msg) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    continue;
                }
            };
            
            if let Err(e) = sender.send(Message::Text(json)).await {
                error!("Failed to send WebSocket message: {}", e);
                break;
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
    
    info!("WebSocket connection closed");
}

async fn handle_client_message(
    text: &str,
    state: &AppState,
    tx: &MessageSender,
    subscriptions: &mut HashMap<String, bool>,
) -> AppResult<()> {
    let client_msg: ClientMessage = serde_json::from_str(text)
        .map_err(|e| crate::utils::error::AppError::bad_request(
            format!("Invalid message format: {}", e)
        ))?;
    
    match client_msg {
        ClientMessage::DataSubscribe { source_id, filters } => {
            info!("Client subscribing to data source: {}", source_id);
            subscriptions.insert(source_id.clone(), true);
            
            // TODO: Implement actual data subscription logic
            // For now, send a dummy update
            let update_msg = ServerMessage::DataUpdate {
                source_id: source_id.clone(),
                data: vec![serde_json::json!({"message": "Subscribed to data updates"})],
            };
            
            tx.send(update_msg).map_err(|e| {
                crate::utils::error::AppError::internal(format!("Failed to send subscription confirmation: {}", e))
            })?;
        }
        
        ClientMessage::DataUnsubscribe { source_id } => {
            info!("Client unsubscribing from data source: {}", source_id);
            subscriptions.remove(&source_id);
        }
        
        ClientMessage::QueryExecute { sql, params } => {
            info!("Client executing query: {}", sql);
            
            // Execute the query
            let query_id = uuid::Uuid::new_v4().to_string();
            
            match execute_websocket_query(state, &sql).await {
                Ok(data) => {
                    let result_msg = ServerMessage::QueryResult {
                        query_id,
                        data,
                        error: None,
                    };
                    
                    tx.send(result_msg).map_err(|e| {
                        crate::utils::error::AppError::internal(format!("Failed to send query result: {}", e))
                    })?;
                }
                Err(e) => {
                    let error_msg = ServerMessage::QueryResult {
                        query_id,
                        data: vec![],
                        error: Some(e.to_string()),
                    };
                    
                    tx.send(error_msg).map_err(|send_err| {
                        crate::utils::error::AppError::internal(format!("Failed to send query error: {}", send_err))
                    })?;
                }
            }
        }
    }
    
    Ok(())
}

async fn execute_websocket_query(
    state: &AppState,
    sql: &str,
) -> AppResult<Vec<serde_json::Value>> {
    let conn = state.db_pool.get_connection();
    let conn_guard = conn.lock().await;
    
    // Basic SQL validation
    let sql_lower = sql.to_lowercase();
    if sql_lower.contains("drop") || sql_lower.contains("delete") || sql_lower.contains("insert") || sql_lower.contains("update") {
        return Err(crate::utils::error::AppError::bad_request(
            "Only SELECT queries are allowed via WebSocket"
        ));
    }
    
    let mut stmt = conn_guard.prepare(sql)?;
    let mut rows = stmt.query([])?;
    
    let column_count = stmt.column_count();
    let columns: Vec<String> = (0..column_count)
        .map(|i| stmt.column_name(i).unwrap_or("unknown").to_string())
        .collect();
    
    let mut data = Vec::new();
    
    while let Some(row) = rows.next()? {
        let mut row_obj = serde_json::Map::new();
        
        for (i, column_name) in columns.iter().enumerate() {
            let value = match row.get_ref(i)? {
                duckdb::types::ValueRef::Null => serde_json::Value::Null,
                duckdb::types::ValueRef::Integer(n) => serde_json::Value::Number(n.into()),
                duckdb::types::ValueRef::Real(f) => serde_json::Value::Number(
                    serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))
                ),
                duckdb::types::ValueRef::Text(s) => serde_json::Value::String(String::from_utf8_lossy(s).to_string()),
                duckdb::types::ValueRef::Blob(_) => serde_json::Value::String("BLOB".to_string()),
            };
            
            row_obj.insert(column_name.clone(), value);
        }
        
        data.push(serde_json::Value::Object(row_obj));
        
        // Limit results to prevent memory issues
        if data.len() >= 1000 {
            warn!("Query result truncated to 1000 rows");
            break;
        }
    }
    
    Ok(data)
}
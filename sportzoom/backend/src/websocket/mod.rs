use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    extract::State,
};
use crate::AppState;
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::info;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, _state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    info!("New WebSocket connection established");
    
    // Spawn task to handle incoming messages
    let send_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("Received message: {}", text);
                    // Handle different message types (chat, reactions, etc.)
                }
                Ok(Message::Close(_)) => {
                    info!("Client disconnected");
                    break;
                }
                Err(e) => {
                    info!("WebSocket error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Wait for send task to complete
    let _ = send_task.await;
    
    info!("WebSocket connection closed");
}

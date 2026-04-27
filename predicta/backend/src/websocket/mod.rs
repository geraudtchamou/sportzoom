use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    State,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::AppState;

/// WebSocket handler for real-time features
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Generate unique user ID for this connection
    let user_id = Uuid::new_v4();
    
    // Subscribe to broadcast channel for real-time updates
    let mut rx = state.redis.subscribe("global").await.unwrap();
    
    // Spawn task to send messages to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    // Handle different message types
                    // - join_room: { "type": "join_room", "room_id": "..." }
                    // - leave_room: { "type": "leave_room", "room_id": "..." }
                    // - chat_message: { "type": "chat_message", "room_id": "...", "content": "..." }
                    // - reaction: { "type": "reaction", "room_id": "...", "emoji": "🔥" }
                    // - prediction: { "type": "prediction", "event_id": "...", "data": {...} }
                    
                    tracing::info!("Received WebSocket message: {}", text);
                    
                    // In production, parse and route messages appropriately
                    // For now, just log them
                }
                Message::Close(_) => {
                    tracing::info!("WebSocket connection closed for user {}", user_id);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = (&mut send_task) => {},
        _ = (&mut recv_task) => {},
    }

    // Cleanup
    send_task.abort();
    recv_task.abort();
    
    tracing::info!("WebSocket connection ended for user {}", user_id);
}

/// Broadcast message to all connected clients in a room
pub async fn broadcast_to_room(
    redis_client: &mut redis::aio::ConnectionManager,
    room_id: &str,
    message: &str,
) -> Result<(), redis::RedisError> {
    redis::cmd("PUBLISH")
        .arg(format!("room:{}", room_id))
        .arg(message)
        .query_async(redis_client)
        .await
}

/// Send message to specific user
pub async fn send_to_user(
    redis_client: &mut redis::aio::ConnectionManager,
    user_id: &Uuid,
    message: &str,
) -> Result<(), redis::RedisError> {
    redis::cmd("PUBLISH")
        .arg(format!("user:{}", user_id))
        .arg(message)
        .query_async(redis_client)
        .await
}

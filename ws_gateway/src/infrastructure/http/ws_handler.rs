use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use uuid::Uuid;
use tracing::info;

use crate::infrastructure::http::state::AppState;
use crate::domain::client::Client;

pub struct DefaultWsHandler;

impl DefaultWsHandler {
    pub fn handle(&self, ws: WebSocketUpgrade, state: State<AppState>) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_connection(socket, state))
    }
}

async fn handle_connection(socket: WebSocket, State(state): State<AppState>) {
    let user_id = Uuid::new_v4();
    let channel_id = Uuid::new_v4();
    let client = Client { user_id, channel_id };

    state.add_client(client.clone()).await;
    info!("User {} connected to channel {}", user_id, channel_id);

    handle_socket(socket, state.clone(), user_id, channel_id).await;

    state.remove_client(user_id).await;
    info!("User {} disconnected", user_id);
}

async fn handle_socket(
    mut socket: WebSocket,
    _state: AppState,
    user_id: Uuid,
    channel_id: Uuid,
) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                info!("recv from user {} on channel {}: {}", user_id, channel_id, text);
                let reply = format!("echo: {}", text);
                if socket.send(Message::Text(reply.into())).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }
}
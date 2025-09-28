use axum::{extract::State, routing::get, Router};
use crate::infrastructure::http::state::AppState;
use crate::infrastructure::http::ws_handler::DefaultWsHandler;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/ws", get(websocket_route))
        .with_state(state)
}

async fn websocket_route(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl axum::response::IntoResponse {
    let handler = DefaultWsHandler;
    handler.handle(ws, State(state))
}
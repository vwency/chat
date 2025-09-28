use tracing_subscriber::EnvFilter;
use ws_gateway::infrastructure::http::router::create_router;
use ws_gateway::infrastructure::http::state::AppState;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let state = AppState::default();
    let app = create_router(state);

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    
    info!("🚀 WebSocket Gateway listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}

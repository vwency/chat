use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("WebSocket error: {0}")]
    WsError(String),

    #[error("NATS error: {0}")]
    NatsError(String),

    #[error("Unknown error")]
    Unknown,
}

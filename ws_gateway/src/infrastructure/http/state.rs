use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::domain::client::Client;

#[derive(Clone, Default)]
pub struct AppState {
    pub connections: Arc<RwLock<HashMap<Uuid, Client>>>,
}

impl AppState {
    pub async fn add_client(&self, client: Client) {
        let mut connections = self.connections.write().await;
        connections.insert(client.user_id, client);
    }

    pub async fn remove_client(&self, user_id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
    }
}
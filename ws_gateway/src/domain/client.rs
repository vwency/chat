use uuid::Uuid;

#[derive(Clone)]
pub struct Client {
    pub user_id: Uuid,
    pub channel_id: Uuid,
}

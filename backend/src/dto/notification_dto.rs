use serde::{Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub message: String,
    pub link: Option<String>,
}

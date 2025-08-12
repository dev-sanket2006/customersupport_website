use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Attachment {
    pub id: Uuid,
    pub file_name: String,
    pub file_url: String,
    pub uploaded_by: Uuid,
    pub message_id: Option<Uuid>,
    pub uploaded_at: DateTime<Utc>,
}

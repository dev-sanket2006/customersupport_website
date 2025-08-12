use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub is_from_customer: bool,
    pub channel: Option<String>,
    pub in_reply_to: Option<Uuid>,
    pub subject: Option<String>,
    pub attachment_ids: Option<Vec<Uuid>>,
    pub message_id: Option<String>,
    pub external_sender_email: Option<String>,
    pub is_email: Option<bool>, // Keep as Option<bool>
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageWithSender {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub is_from_customer: bool,
    pub channel: Option<String>,
    pub in_reply_to: Option<Uuid>,
    pub subject: Option<String>,
    pub attachment_ids: Option<Vec<Uuid>>,
    pub message_id: Option<String>,
    pub external_sender_email: Option<String>,
    pub is_email: Option<bool>,
    pub created_at: chrono::NaiveDateTime,
    pub sender_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageInput {
    pub content: String,
    pub is_from_customer: bool,
    pub channel: Option<String>,
    pub in_reply_to: Option<Uuid>,
    pub subject: Option<String>,
    pub attachment_ids: Option<Vec<Uuid>>,
    pub message_id: Option<String>,
    pub external_sender_email: Option<String>,
    pub is_email: bool,
}
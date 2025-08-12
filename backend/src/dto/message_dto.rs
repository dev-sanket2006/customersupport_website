use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
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
    pub is_email: Option<bool>, // ✅ Include here too
}


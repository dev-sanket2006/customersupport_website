use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

// ✅ New struct for frontend responses (with author email)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NoteWithAuthor {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub author_email: String, // 👈 Extra field for UI
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateNoteInput {
    pub ticket_id: Uuid,
    pub content: String,
}
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub ticket_id: Uuid,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub note_id: Uuid,
    pub content: String,
}

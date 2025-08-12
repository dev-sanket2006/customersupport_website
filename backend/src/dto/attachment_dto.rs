use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UploadAttachmentRequest {
    pub file_name: String,
    pub file_url: String, // Assume URL returned from client upload (e.g., S3)
    pub uploaded_by: Uuid,
    pub message_id: Option<Uuid>,
}

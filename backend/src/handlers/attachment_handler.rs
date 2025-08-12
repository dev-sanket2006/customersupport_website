use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::query_as_unchecked;
use crate::{
    dto::attachment_dto::UploadAttachmentRequest,
    models::attachment::Attachment,
    state::SharedState,
};

pub async fn upload_attachment(
    State(state): State<SharedState>,
    Json(payload): Json<UploadAttachmentRequest>,
) -> Result<Json<Attachment>, StatusCode> {
    let record = query_as_unchecked!(
        Attachment,
        r#"
        INSERT INTO attachments (file_name, file_url, uploaded_by, message_id)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        payload.file_name,
        payload.file_url,
        payload.uploaded_by,
        payload.message_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error uploading attachment: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(record))
}

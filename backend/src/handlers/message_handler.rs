use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sqlx::query_as_unchecked;
use uuid::Uuid;

use crate::{
    dto::message_dto::CreateMessageRequest,
    models::message::{Message, MessageWithSender},
    state::SharedState,
    services::collaboration_service::{get_messages_by_ticket},
};

/// POST /messages
pub async fn send_message(
    State(state): State<SharedState>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    let now = Utc::now();

    let message = query_as_unchecked!(
        Message,
        r#"
        INSERT INTO messages (
            ticket_id,
            sender_id,
            content,
            is_from_customer,
            channel,
            in_reply_to,
            subject,
            attachment_ids,
            message_id,
            external_sender_email,
            is_email,
            created_at
        )
        VALUES (
            $1, $2, $3, $4,
            $5, $6, $7, $8,
            $9, $10, $11, $12
        )
        RETURNING
            id, ticket_id, sender_id, content, is_from_customer,
            channel, in_reply_to, subject, attachment_ids,
            message_id, external_sender_email, is_email, created_at
        "#,
        payload.ticket_id,
        payload.sender_id,
        payload.content,
        payload.is_from_customer,
        payload.channel,
        payload.in_reply_to,
        payload.subject,
        payload.attachment_ids.as_ref().map(|ids| &**ids),
        payload.message_id,
        payload.external_sender_email,
        payload.is_email,
        now
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error sending message: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // ✅ Broadcast via WebSocket to subscribed clients
    if let Some(tx) = state.ws_channels.read().await.get(&message.ticket_id) {
        if let Ok(msg_json) = serde_json::to_string(&message) {
            let _ = tx.send(msg_json); // ignore error if no receivers
        }
    }

    Ok(Json(message))
}

/// GET /messages/:ticket_id
pub async fn get_messages(
    State(state): State<SharedState>,
    Path(ticket_id): Path<Uuid>,
) -> Result<Json<Vec<MessageWithSender>>, StatusCode> {
    let messages = get_messages_by_ticket(&state.db, ticket_id)
        .await
        .map_err(|err| {
            tracing::error!("Error fetching messages: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(messages))
}

use crate::models::message::{CreateMessageInput, Message, MessageWithSender};
use crate::state::SharedState;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;
use tracing::{info, warn};

/// Adds a new message to a ticket and broadcasts it via WebSocket.
pub async fn add_message_to_ticket(
    pool: &PgPool,
    ticket_id: Uuid,
    sender_id: Uuid,
    input: CreateMessageInput,
    state: Option<SharedState>, // Optional state for WebSocket broadcasting
) -> Result<Message, sqlx::Error> {
    let message = sqlx::query_as_unchecked!(
        Message,
        r#"
        INSERT INTO messages (
            id,
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
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9,
            $10, $11, $12, $13
        )
        RETURNING 
            id, ticket_id, sender_id, content, is_from_customer,
            channel, in_reply_to, subject, attachment_ids,
            message_id, external_sender_email, is_email, created_at
        "#,
        Uuid::new_v4(),
        ticket_id,
        sender_id,
        input.content,
        input.is_from_customer,
        input.channel,
        input.in_reply_to,
        input.subject,
        input.attachment_ids.as_ref().map(|ids| &**ids),
        input.message_id,
        input.external_sender_email,
        input.is_email,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    // Broadcast the new message via WebSocket if state is provided
    if let Some(app_state) = state {
        broadcast_message_to_websocket(&app_state, &message).await;
    }

    Ok(message)
}

/// Broadcasts a message to WebSocket clients connected to the ticket
async fn broadcast_message_to_websocket(state: &SharedState, message: &Message) {
    let channels = state.ws_channels.read().await;
    
    if let Some(tx) = channels.get(&message.ticket_id) {
        // Create a JSON representation of the message for broadcasting
        let ws_message = json!({
            "id": message.id,
            "ticket_id": message.ticket_id,
            "sender_id": message.sender_id,
            "content": message.content,
            "is_from_customer": message.is_from_customer,
            "channel": message.channel,
            "created_at": message.created_at,
            "sender_name": if message.is_from_customer { 
                message.external_sender_email.as_deref().unwrap_or("Customer") 
            } else { 
                "Support Agent" 
            }
        });

        if let Ok(json_str) = serde_json::to_string(&ws_message) {
            match tx.send(json_str) {
                Ok(_) => {
                    info!("Broadcasted message {} to WebSocket clients for ticket {}", 
                          message.id, message.ticket_id);
                }
                Err(e) => {
                    warn!("Failed to broadcast message to WebSocket: {:?}", e);
                }
            }
        }
    }
}

/// Retrieves all messages for a specific ticket along with sender details.
pub async fn get_messages_by_ticket(
    pool: &PgPool,
    ticket_id: Uuid,
) -> Result<Vec<MessageWithSender>, sqlx::Error> {
    let messages = sqlx::query_as_unchecked!(
        MessageWithSender,
        r#"
        SELECT 
            m.id,
            m.ticket_id,
            m.sender_id,
            m.content,
            m.is_from_customer,
            m.channel,
            m.in_reply_to,
            m.subject,
            m.attachment_ids,
            m.message_id,
            m.external_sender_email,
            m.is_email,
            m.created_at,
            COALESCE(m.external_sender_email, u.name) AS sender_name
        FROM messages m
        LEFT JOIN users u ON m.sender_id = u.id
        WHERE m.ticket_id = $1
        ORDER BY m.created_at ASC
        "#,
        ticket_id
    )
    .fetch_all(pool)
    .await?;

    Ok(messages)
}
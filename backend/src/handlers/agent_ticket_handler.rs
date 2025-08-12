use axum::{
    extract::{Path, Request, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    models::ticket::Ticket,
    services::notification_services::notify_user,
    state::SharedState,
    utils::jwt::Claims,
};

#[derive(serde::Deserialize)]
pub struct ReplyPayload {
    pub ticket_id: Uuid,
    pub content: String,
}

pub async fn list_or_reply_agent_tickets(
    Path(agent_id): Path<Uuid>,
    State(state): State<SharedState>,
    req: Request,
    Json(payload): Json<Option<ReplyPayload>>,
) -> Response {
    let Some(claims) = req.extensions().get::<Claims>() else {
        return (StatusCode::UNAUTHORIZED, "Missing authentication").into_response();
    };

    if claims.role != "agent" {
        return (StatusCode::FORBIDDEN, "Only agents can access this route").into_response();
    }

    if claims.sub != agent_id {
        return (StatusCode::FORBIDDEN, "Agent ID mismatch").into_response();
    }

    let method = req.method();

    if method == Method::GET {
        match query_as::<_, Ticket>(
            r#"
            SELECT id, subject, description,
                   status, priority, assigned_to,
                   created_at, updated_at,
                   customer_email, user_id
            FROM tickets
            WHERE assigned_to = $1
            "#,
        )
        .bind(agent_id)
        .fetch_all(&state.db)
        .await
        {
            Ok(tickets) => Json(tickets).into_response(),
            Err(err) => {
                tracing::error!("DB error fetching tickets: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch tickets").into_response()
            }
        }
    } else if method == Method::POST {
        let Some(reply) = payload else {
            return (StatusCode::BAD_REQUEST, "Missing reply payload").into_response();
        };

        if let Err(err) = query!(
            r#"
            INSERT INTO messages (id, ticket_id, sender_id, content, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            Uuid::new_v4(),
            reply.ticket_id,
            claims.sub,
            reply.content,
            Utc::now()
        )
        .execute(&state.db)
        .await
        {
            tracing::error!("Failed to insert reply message: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send reply").into_response();
        }

        let ticket = match query_as::<_, Ticket>(
            r#"
            SELECT id, subject, description,
                   status, priority, assigned_to,
                   created_at, updated_at,
                   customer_email, user_id
            FROM tickets
            WHERE id = $1
            "#,
        )
        .bind(reply.ticket_id)
        .fetch_one(&state.db)
        .await
        {
            Ok(t) => t,
            Err(err) => {
                tracing::error!("Ticket fetch failed: {:?}", err);
                return (StatusCode::NOT_FOUND, "Ticket not found").into_response();
            }
        };

        if let Some(user_id) = ticket.user_id {
            if let Err(err) = notify_user(
                &state.db,
                user_id,
                &format!("Agent replied to your ticket: {}", ticket.subject),
                Some(format!("/dashboard/ticket/{}", ticket.id)),
            )
            .await
            {
                tracing::error!("Failed to notify user: {:?}", err);
            }
        } else {
            tracing::warn!("Ticket {} has no user_id. Skipping notification.", ticket.id);
        }

        (StatusCode::OK, "Reply sent and user notified").into_response()
    } else {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Use GET to list or POST to reply",
        )
            .into_response()
    }
}

use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    middleware::auth::AuthUser,
    models::{note::Note, ticket::Ticket},
    state::{AppState, SharedState},
};

/// Mount all agent ticket-related routes here.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/agent/tickets", get(list_tickets_for_agent))
        .route("/agent/tickets/{id}", get(get_ticket_by_id_for_agent)) // ✅ Fixed
        .route("/agent/tickets/{id}/reply", post(reply_to_ticket)) // ✅ Fixed
        .with_state(state)
}

// === Handler: GET /agent/tickets ===
async fn list_tickets_for_agent(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<Ticket>>, StatusCode> {
    let agent_id = user.0.id;

    let tickets = query_as::<_, Ticket>(
        r#"
    SELECT id, subject, description,
           customer_email, assigned_to,
           priority, status,
           created_at, updated_at,
           user_id
    FROM tickets
    WHERE assigned_to = $1
    "#,
    )
    .bind(agent_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error fetching tickets: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tickets))
}

// === Handler: GET /agent/tickets/{id} ===
async fn get_ticket_by_id_for_agent(
    Path(ticket_id): Path<Uuid>,
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Ticket>, StatusCode> {
    let agent_id = user.0.id;

    let ticket = query_as::<_, Ticket>(
        r#"
    SELECT id, subject, description,
           customer_email, assigned_to,
           priority, status,
           created_at, updated_at,
           user_id
    FROM tickets
    WHERE id = $1 AND assigned_to = $2
    "#,
    )
    .bind(ticket_id)
    .bind(agent_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error fetching ticket by ID: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match ticket {
        Some(t) => Ok(Json(t)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// === Handler: POST /agent/tickets/{id}/reply ===
#[derive(Debug, Deserialize)]
struct ReplyInput {
    message: String,
}

#[derive(Debug, Serialize)]
struct ReplyResponse {
    success: bool,
    note: String,
}

async fn reply_to_ticket(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<ReplyInput>,
) -> Result<Json<ReplyResponse>, StatusCode> {
    let author_id = user.0.id;

    let result = query!(
        r#"
        INSERT INTO notes (ticket_id, author_id, content)
        VALUES ($1, $2, $3)
        "#,
        ticket_id,
        author_id,
        payload.message
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(Json(ReplyResponse {
            success: true,
            note: format!("Reply to ticket #{} saved.", ticket_id),
        })),
        Err(e) => {
            tracing::error!("Error inserting note: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

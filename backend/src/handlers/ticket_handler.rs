use axum::{
    extract::{Path, State, Request},
    http::StatusCode,
    Json,
};
use crate::{
    dto::ticket_dto::{CreateTicketRequest, UpdateTicketRequest},
    models::ticket::{Ticket, TicketPriority},
    state::SharedState,
    utils::jwt::Claims,
    services::notification_services::notify_user,
    models::user::User,
};
use uuid::Uuid;
use chrono::Utc;
use validator::Validate;
use sqlx::{query, query_as};

/// Create a new ticket
pub async fn create_ticket(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    if let Err(errors) = payload.validate() {
        tracing::warn!("Validation failed for create_ticket: {:?}", errors);
        return Err(StatusCode::BAD_REQUEST);
    }

    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        INSERT INTO tickets (subject, description, priority, customer_email)
        VALUES ($1, $2, $3::ticket_priority, $4)
        RETURNING *
        "#,
    )
    .bind(&payload.subject)
    .bind(&payload.description)
    .bind(payload.priority.unwrap_or(TicketPriority::Medium))
    .bind(&payload.customer_email)
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error creating ticket: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // 🔔 Notify all agents and admins
    let users = sqlx::query_as::<_, User>(
        "SELECT id, role FROM users WHERE role IN ('agent', 'admin')"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error fetching users for notification: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for user in users {
        let _ = notify_user(
            &state.db,
            user.id,
            &format!("New ticket created: {}", ticket.subject),
            Some(format!("/dashboard/ticket/{}", ticket.id)),
        )
        .await;
    }

    Ok(Json(ticket))
}

/// Get ticket by ID
pub async fn get_ticket_by_id(
    State(state): State<SharedState>,
    Path(ticket_id): Path<Uuid>,
    req: Request,
) -> Result<Json<Ticket>, StatusCode> {
    let Some(claims) = req.extensions().get::<Claims>() else {
        tracing::warn!("Missing JWT claims in request");
        return Err(StatusCode::UNAUTHORIZED);
    };

    let ticket = sqlx::query_as::<_, Ticket>("SELECT * FROM tickets WHERE id = $1")
        .bind(ticket_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|err| {
            tracing::error!("DB error fetching ticket: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let Some(ticket) = ticket else {
        return Err(StatusCode::NOT_FOUND);
    };

    match claims.role.as_str() {
        "user" => {
            if Some(claims.email.clone()) != ticket.customer_email {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "agent" => {
            if ticket.assigned_to != Some(claims.sub) {
                return Err(StatusCode::FORBIDDEN);
            }
        }
        "admin" => {}
        _ => return Err(StatusCode::FORBIDDEN),
    }

    Ok(Json(ticket))
}

/// List tickets for the authenticated user
pub async fn list_tickets(
    State(state): State<SharedState>,
    req: Request,
) -> Result<Json<Vec<Ticket>>, StatusCode> {
    let Some(claims) = req.extensions().get::<Claims>() else {
        tracing::warn!("Missing JWT claims in request");
        return Err(StatusCode::UNAUTHORIZED);
    };

    let tickets = query_as::<_, Ticket>(
        "SELECT * FROM tickets WHERE customer_email = $1"
    )
    .bind(&claims.email)
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error listing user tickets: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tickets))
}

/// Update ticket by ID
pub async fn update_ticket(
    State(state): State<SharedState>,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<UpdateTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    if let Err(errors) = payload.validate() {
        tracing::warn!("Validation failed for update_ticket: {:?}", errors);
        return Err(StatusCode::BAD_REQUEST);
    }

    let ticket = sqlx::query_as::<_, Ticket>(
        r#"
        UPDATE tickets
        SET
            subject = COALESCE($1, subject),
            description = COALESCE($2, description),
            status = COALESCE($3, status),
            priority = COALESCE($4, priority),
            assigned_to = COALESCE($5, assigned_to),
            updated_at = $6
        WHERE id = $7
        RETURNING *
        "#,
    )
    .bind(&payload.subject)
    .bind(&payload.description)
    .bind(&payload.status)
    .bind(&payload.priority)
    .bind(&payload.assigned_to)
    .bind(Utc::now())
    .bind(ticket_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error updating ticket: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(updated_ticket) = &ticket {
        // Notify agent if assigned
        if let Some(agent_id) = updated_ticket.assigned_to {
            let _ = notify_user(
                &state.db,
                agent_id,
                &format!("Ticket updated: {}", updated_ticket.subject),
                Some(format!("/dashboard/ticket/{}", updated_ticket.id)),
            ).await;
        }

        // Notify customer if email found in users
        if let Some(email) = &updated_ticket.customer_email {
            if let Ok(user) = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE email = $1"
            )
            .bind(email)
            .fetch_one(&state.db)
            .await
            {
                let _ = notify_user(
                    &state.db,
                    user.id,
                    &format!("Your ticket was updated: {}", updated_ticket.subject),
                    Some(format!("/dashboard/ticket/{}", updated_ticket.id)),
                ).await;
            }
        }
    }

    match ticket {
        Some(t) => Ok(Json(t)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Delete a ticket
pub async fn delete_ticket(
    State(state): State<SharedState>,
    Path(ticket_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM tickets WHERE id = $1",
        ticket_id
    )
    .execute(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error deleting ticket: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

/// Admin-only: List all tickets
pub async fn admin_list_tickets(
    State(state): State<SharedState>,
    req: Request,
) -> Result<Json<Vec<Ticket>>, StatusCode> {
    let Some(claims) = req.extensions().get::<Claims>() else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let tickets = sqlx::query_as::<_, Ticket>(
        "SELECT * FROM tickets"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error listing tickets (admin): {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tickets))
}

/// Assign a ticket to an agent (admin/agent only)
pub async fn assign_ticket(
    Path((ticket_id, agent_id)): Path<(Uuid, Uuid)>,
    State(state): State<SharedState>,
    req: Request,
) -> Result<StatusCode, StatusCode> {
    let Some(claims) = req.extensions().get::<Claims>() else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if claims.role != "admin" && claims.role != "agent" {
        return Err(StatusCode::FORBIDDEN);
    }

    let result = sqlx::query(
        r#"
        UPDATE tickets
        SET assigned_to = $1, status = 'InProgress'
        WHERE id = $2
        "#,
    )
    .bind(agent_id)
    .bind(ticket_id)
    .execute(&state.db)
    .await;

    match result {
        Ok(query_result) if query_result.rows_affected() == 1 => {
            let _ = notify_user(
                &state.db,
                agent_id,
                "You have been assigned a new ticket.",
                Some(format!("/dashboard/ticket/{}", ticket_id)),
            ).await;
            Ok(StatusCode::OK)
        }
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Error assigning ticket: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

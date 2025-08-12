use crate::models::ticket::{
    CreateTicketInput, Ticket, TicketPriority, TicketStatus, UpdateTicketInput,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Create a new ticket
pub async fn create_ticket(
    pool: &PgPool,
    input: CreateTicketInput,
    agent_id: Uuid,
    user_id: Uuid,
) -> Result<Ticket, sqlx::Error> {
    let now = Utc::now();
    let ticket = sqlx::query_as_unchecked!(
        Ticket,
        r#"
        INSERT INTO tickets (
            id, subject, description, status, priority,
            assigned_to, created_at, updated_at,
            customer_email, user_id
        )
        VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $7,
            $8, $9
        )
        RETURNING id, subject, description, status as "status: _", priority as "priority: _",
                  assigned_to, created_at, updated_at, customer_email, user_id
        "#,
        Uuid::new_v4(),
        input.subject,
        input.description,
        TicketStatus::Open as TicketStatus,
        input.priority as TicketPriority,
        agent_id,
        now,
        input.customer_email,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(ticket)
}

/// Get all tickets
pub async fn get_all_tickets(pool: &PgPool) -> Result<Vec<Ticket>, sqlx::Error> {
    let tickets = sqlx::query_as_unchecked!(
        Ticket,
        r#"
        SELECT id, subject, description,
               status as "status: _", priority as "priority: _",
               assigned_to, created_at, updated_at,
               customer_email, user_id
        FROM tickets
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(tickets)
}

/// Get a ticket by its ID
pub async fn get_ticket_by_id(pool: &PgPool, ticket_id: Uuid) -> Result<Ticket, sqlx::Error> {
    sqlx::query_as_unchecked!(
        Ticket,
        r#"
        SELECT id, subject, description,
               status as "status: _", priority as "priority: _",
               assigned_to, created_at, updated_at,
               customer_email, user_id
        FROM tickets
        WHERE id = $1
        "#,
        ticket_id
    )
    .fetch_one(pool)
    .await
}

/// Update a ticket's status
pub async fn update_ticket_status(
    pool: &PgPool,
    ticket_id: Uuid,
    input: UpdateTicketInput,
) -> Result<Ticket, sqlx::Error> {
    let now = Utc::now();

    let updated = sqlx::query_as_unchecked!(
        Ticket,
        r#"
        UPDATE tickets
        SET status = $1, updated_at = $2
        WHERE id = $3
        RETURNING id, subject, description,
                  status as "status: _", priority as "priority: _",
                  assigned_to, created_at, updated_at,
                  customer_email, user_id
        "#,
        input.status as TicketStatus,
        now,
        ticket_id
    )
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Delete a ticket
pub async fn delete_ticket(pool: &PgPool, ticket_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM tickets WHERE id = $1", ticket_id)
        .execute(pool)
        .await?;

    Ok(())
}

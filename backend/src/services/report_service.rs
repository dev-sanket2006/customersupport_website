use sqlx::{PgPool, query_as_unchecked};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct TicketOverview {
    pub total: i64,
    pub open: i64,
    pub closed: i64,
    pub in_progress: i64,
}

#[derive(Serialize)]
pub struct TicketsByAgent {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub ticket_count: i64,
}

#[derive(Serialize)]
pub struct TicketStatusCount {
    pub status: String,
    pub count: i64,
}

pub async fn get_ticket_overview(pool: &PgPool) -> Result<TicketOverview, sqlx::Error> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tickets")
        .fetch_one(pool)
        .await?;

    let open: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tickets WHERE status = 'open'")
        .fetch_one(pool)
        .await?;

    let closed: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tickets WHERE status = 'closed'")
        .fetch_one(pool)
        .await?;

    let in_progress: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM tickets WHERE status = 'in_progress'")
            .fetch_one(pool)
            .await?;

    Ok(TicketOverview {
        total: total.0,
        open: open.0,
        closed: closed.0,
        in_progress: in_progress.0,
    })
}

pub async fn get_tickets_by_agent(pool: &PgPool) -> Result<Vec<TicketsByAgent>, sqlx::Error> {
    let results = query_as_unchecked!(
        TicketsByAgent,
        r#"
        SELECT u.id AS agent_id, u.name AS agent_name, COUNT(t.id)::BIGINT AS ticket_count
        FROM users u
        JOIN tickets t ON u.id = t.assigned_to
        GROUP BY u.id, u.name
        ORDER BY ticket_count DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

pub async fn get_ticket_status_count(pool: &PgPool) -> Result<Vec<TicketStatusCount>, sqlx::Error> {
    let results = query_as_unchecked!(
        TicketStatusCount,
        r#"
        SELECT status::TEXT AS status, COUNT(*)::BIGINT AS count
        FROM tickets
        GROUP BY status
        ORDER BY count DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

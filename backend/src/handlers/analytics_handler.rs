use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::{query_scalar, Postgres};
use crate::{
    models::analytics::TicketStats,
    state::SharedState,
};

pub async fn ticket_summary(
    State(state): State<SharedState>,
) -> Result<Json<TicketStats>, StatusCode> {
    let total: i64 = query_scalar::<Postgres, i64>("SELECT COUNT(*) FROM tickets")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let open: i64 = query_scalar::<Postgres, i64>("SELECT COUNT(*) FROM tickets WHERE status = 'open'::ticket_status")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let in_progress: i64 = query_scalar::<Postgres, i64>("SELECT COUNT(*) FROM tickets WHERE status = 'in_progress'::ticket_status")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let resolved: i64 = query_scalar::<Postgres, i64>("SELECT COUNT(*) FROM tickets WHERE status = 'resolved'::ticket_status")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let closed: i64 = query_scalar::<Postgres, i64>("SELECT COUNT(*) FROM tickets WHERE status = 'closed'::ticket_status")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TicketStats {
        total,
        open,
        in_progress,
        resolved,
        closed,
    }))
}

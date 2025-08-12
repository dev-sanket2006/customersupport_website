use axum::{Router, routing::get};
use crate::handlers::analytics_handler::ticket_summary;
use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/analytics/summary", get(ticket_summary))
        .with_state(state) // ✅ Required
}

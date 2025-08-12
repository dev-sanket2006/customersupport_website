use axum::{routing::get, Router};
use crate::{
    state::SharedState,
    handlers::ws_handler::handle_ws_upgrade_query, // ✅ Correct path to ws_handler
};

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/ws/tickets/{ticket_id}", get(handle_ws_upgrade_query)) // ✅ Updated to use curly braces
        .with_state(state)
}
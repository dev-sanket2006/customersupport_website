use axum::{Router, routing::{post, get}};
use crate::handlers::note_handler::{add_note, get_notes};
use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/notes", post(add_note).get(get_notes)) // ✅ Handles both GET and POST
        .with_state(state)
}

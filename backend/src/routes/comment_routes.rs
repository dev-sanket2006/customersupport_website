use axum::{
    Router,
    routing::{post, get, delete, put},
};

use crate::handlers::comment_handler::{
    add_comment,
    get_comments_by_note,
    delete_comment,
    update_comment,
};

use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/comments", post(add_comment)) // POST /comments
        .route("/comments/by-note/{note_id}", get(get_comments_by_note)) // ✅ curly braces
        .route("/comments/{comment_id}", delete(delete_comment))         // ✅ curly braces
        .route("/comments/{comment_id}", put(update_comment))            // ✅ curly braces
        .with_state(state)
}

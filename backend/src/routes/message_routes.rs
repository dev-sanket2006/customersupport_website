use axum::{
    Router,
    routing::{post, get}, // make sure this is here
};

use crate::handlers::message_handler::{send_message, get_messages};
use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/messages", post(send_message))
        .route("/messages/{ticket_id}", get(get_messages)) // ✅ Fixed path syntax
        .with_state(state)
}

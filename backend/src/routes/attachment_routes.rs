use axum::{
    routing::post,
    Router,
};

use crate::handlers::attachment_handler::upload_attachment;
use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/attachments", post(upload_attachment))
        .with_state(state) // ✅ this tells Axum to inject SharedState into handlers
}

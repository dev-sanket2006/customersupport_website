use axum::{
    Router,
    routing::{post, get},
};
use crate::{
    handlers::notification_handler::{create_notification, get_notifications},
    state::SharedState,
};

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/notifications", post(create_notification))
        .route("/notifications", get(get_notifications)) // ✅ fetch notifications for authenticated user
        .with_state(state)
}

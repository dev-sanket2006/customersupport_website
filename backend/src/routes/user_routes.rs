use axum::{routing::get, Router};
use crate::{
    handlers::user_handler::get_agents,
    state::SharedState,
};

pub fn user_routes(state: SharedState) -> Router {
    Router::new()
        .route("/agents", get(get_agents))
        .with_state(state) // ✅ This adds the app state needed by get_agents
}
pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/agents", get(get_agents))
        .with_state(state)
}

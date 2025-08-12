use axum::{Router, routing::post};
use crate::handlers::auth_handler::{register, login};
use crate::state::SharedState;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(state) // ✅ required for State<SharedState>
}

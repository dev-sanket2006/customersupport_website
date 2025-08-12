use axum::{extract::State, Json};
use crate::{state::SharedState, middleware::auth::AuthUser};

pub async fn protected_handler(
    State(_state): State<SharedState>,
    AuthUser(user): AuthUser,
) -> Json<String> {
    Json(format!("Hello, {}!", user.email))
}

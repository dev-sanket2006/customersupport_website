use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    body::Body,
    response::Response,
};
use crate::{
    state::SharedState,
    utils::jwt::decode_token,
};

pub async fn require_auth(
    State(state): State<SharedState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract Bearer token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Decode token
    let claims = decode_token(auth_header, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add email to request extensions
    req.extensions_mut().insert(claims.email);

    // Call next middleware or handler
    Ok(next.run(req).await)
}

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::utils::jwt::decode_token;
use std::env;

/// Accepts a list of allowed roles (e.g., `["admin", "agent"]`)
pub async fn require_roles(
    mut req: Request<Body>,
    next: Next,
    allowed_roles: &'static [&'static str],
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => Some(&header[7..]),
        _ => None,
    };

    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "super_secret_key_change_this".to_string());

    let claims = decode_token(token, &secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let role = claims.role.trim().to_lowercase();
    let allowed: Vec<String> = allowed_roles.iter().map(|r| r.to_string()).collect();

    println!("Decoded role: '{}'", role);
    println!("Allowed roles: {:?}", allowed);

    if !allowed.contains(&role) {
        println!("Access denied for role: '{}'", role);
        return Err(StatusCode::FORBIDDEN);
    }

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

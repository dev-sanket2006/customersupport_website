use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::{
    models::user::PublicUser,
    state::SharedState,
    utils::jwt::{decode_token, Claims},
};
use crate::state::AppState;


#[derive(Debug)]
pub struct AuthUser(pub PublicUser);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync + std::ops::Deref<Target = AppState>,  
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,    
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts 
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims: Claims = decode_token(token, &state.config.jwt_secret)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let user = PublicUser {
            id: claims.sub,
            name: "".to_string(),
            email: claims.email,
            role: claims.role,
            is_active: Some(true),
            created_at: None,
            updated_at: None,
        };

        Ok(AuthUser(user))
    }
}
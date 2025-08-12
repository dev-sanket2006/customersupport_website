use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::query_as_unchecked;
use uuid::Uuid;
use crate::{
    dto::auth_dto::{RegisterRequest, LoginRequest, AuthResponse},
    models::user::User,
    state::SharedState,
    utils::{hash::hash_password, hash::verify_password, jwt::create_token},
};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let uuid = Uuid::new_v4();
    let password_hash = hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = query_as_unchecked!(
        User,
        r#"
        INSERT INTO users (id, name, email, password_hash, role)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        uuid,
        payload.name,
        payload.email,
        password_hash,
        payload.role  // ← USE ROLE FROM REQUEST
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("Register DB error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let token = create_token(
        user.id,
        user.email.clone(),
        user.role.clone(),
        &state.config.jwt_secret,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = query_as_unchecked!(
        User,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("Login DB error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = match user {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !user.is_active {
        return Err(StatusCode::FORBIDDEN); // User is disabled
    }

    let token = create_token(
        user.id,
        user.email.clone(),
        user.role.clone(), // ✅ Fixed
        &state.config.jwt_secret,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AuthResponse { token }))
}

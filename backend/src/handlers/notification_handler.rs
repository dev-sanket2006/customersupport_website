use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use uuid::Uuid;

use crate::{
    dto::notification_dto::CreateNotificationRequest,
    models::notification::Notification,
    state::SharedState,
    utils::jwt::decode_token,
};
use crate::services::notification_services::notify_user;

/// Create a new notification (used by backend services or directly via API)
pub async fn create_notification(
    State(state): State<SharedState>,
    Json(payload): Json<CreateNotificationRequest>,
) -> Result<Json<Notification>, StatusCode> {
    let saved = notify_user(
        &state.db,
        payload.user_id,
        &payload.message,
        payload.link.clone(),
    )
    .await
    .map_err(|err| {
        tracing::error!("DB error creating notification: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(saved))
}

/// Get all notifications for the authenticated user
pub async fn get_notifications(
    State(state): State<SharedState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<Vec<Notification>>, StatusCode> {
    let token = bearer.token();

    let claims = decode_token(token, &state.config.jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims.sub;

    let notifications = sqlx::query_as_unchecked!(
        Notification,
        r#"
        SELECT id, user_id, message, is_read, link, created_at
        FROM notifications
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error fetching notifications: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(notifications))
}

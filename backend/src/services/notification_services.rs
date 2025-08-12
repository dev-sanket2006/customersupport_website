use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::notification::Notification;

/// Notify a single user
pub async fn notify_user(
    pool: &PgPool,
    user_id: Uuid,
    message: &str,
    link: Option<String>,
) -> Result<Notification, sqlx::Error> {
    let notif = sqlx::query_as_unchecked!(
        Notification,
        r#"
        INSERT INTO notifications (user_id, message, link, is_read, created_at)
        VALUES ($1, $2, $3, false, $4)
        RETURNING id, user_id, message, is_read, link, created_at
        "#,
        user_id,
        message,
        link,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(notif)
}

/// Notify multiple users
pub async fn notify_users(
    pool: &PgPool,
    user_ids: &[Option<Uuid>],
    message: &str,
    link: Option<String>,
) -> Result<(), sqlx::Error> {
    for user_id_opt in user_ids {
        if let Some(user_id) = user_id_opt {
            let _ = notify_user(pool, *user_id, message, link.clone()).await?;
        }
    }
    Ok(())
}


/// Get notifications for a user
pub async fn get_notifications_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Notification>, sqlx::Error> {
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
    .fetch_all(pool)
    .await?;

    Ok(notifications)
}

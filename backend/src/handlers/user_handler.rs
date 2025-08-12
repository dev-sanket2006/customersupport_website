use axum::{extract::State, Json};
use axum::http::StatusCode;
use sqlx::query;
use crate::{
    models::user::PublicUser,
    state::SharedState,
};

pub async fn get_agents(
    State(state): State<SharedState>,
) -> Result<Json<Vec<PublicUser>>, StatusCode> {
    let rows = query!(
        r#"
        SELECT 
            id, name, email, role, is_active, created_at, updated_at
        FROM users 
        WHERE role = 'agent'
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Manually map to PublicUser
    let agents = rows
        .into_iter()
        .map(|row| PublicUser {
            id: row.id,
            name: row.name,
            email: row.email,
            role: row.role,
            is_active: row.is_active,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .collect();

    Ok(Json(agents))
}

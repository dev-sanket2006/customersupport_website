use axum::{extract::{State, Path}, Json};
use axum::http::StatusCode;
use sqlx::query_as_unchecked;
use uuid::Uuid;
use serde::Deserialize;

use crate::{
    dto::note_dto::CreateCommentRequest,
    models::comment::Comment,
    state::SharedState,
    middleware::auth::AuthUser, // ✅ Pull authenticated user from JWT
};

#[derive(Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

// ✅ POST /comments - Create new comment (auth required)
pub async fn add_comment(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,                          // ✅ Extract user from token
    Json(payload): Json<CreateCommentRequest>,
) -> Result<Json<Comment>, StatusCode> {
    let comment = query_as_unchecked!(
        Comment,
        r#"
        INSERT INTO comments (note_id, author_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        payload.note_id,
        user.id,                                        // ✅ Use ID from token
        payload.content
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error creating comment: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(comment))
}

// ✅ GET /comments/{note_id} - All comments for a note
pub async fn get_comments_by_note(
    State(state): State<SharedState>,
    Path(note_id): Path<Uuid>,
) -> Result<Json<Vec<Comment>>, StatusCode> {
    let comments = query_as_unchecked!(
        Comment,
        r#"
        SELECT * FROM comments
        WHERE note_id = $1
        ORDER BY created_at ASC
        "#,
        note_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error fetching comments: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(comments))
}

// ✅ DELETE /comments/{comment_id}
pub async fn delete_comment(
    State(state): State<SharedState>,
    Path(comment_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        r#"
        DELETE FROM comments
        WHERE id = $1
        "#,
        comment_id
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => {
            tracing::error!("DB error deleting comment: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ✅ PUT /comments/{comment_id}
pub async fn update_comment(
    State(state): State<SharedState>,
    Path(comment_id): Path<Uuid>,
    Json(payload): Json<UpdateCommentRequest>,
) -> Result<Json<Comment>, StatusCode> {
    let comment = query_as_unchecked!(
        Comment,
        r#"
        UPDATE comments
        SET content = $1
        WHERE id = $2
        RETURNING *
        "#,
        payload.content,
        comment_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error updating comment: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(comment))
}

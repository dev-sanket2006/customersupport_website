use axum::{extract::{State, Path}, Json};
use axum::http::StatusCode;

use crate::{
    middleware::auth::AuthUser,
    dto::note_dto::{CreateNoteRequest, CreateCommentRequest},
    models::{note::{Note, NoteWithAuthor}, comment::Comment},
    state::SharedState,
};
use sqlx::query_as_unchecked;

/// POST /notes - Add a new note and return it with author_email
pub async fn add_note(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateNoteRequest>,
) -> Result<Json<NoteWithAuthor>, StatusCode> {
    let note = query_as_unchecked!(
        NoteWithAuthor,
        r#"
        INSERT INTO notes (ticket_id, author_id, content)
        VALUES ($1, $2, $3)
        RETURNING id, ticket_id, author_id, content, created_at,
                  (SELECT email FROM users WHERE id = $2) AS author_email
        "#,
        payload.ticket_id,
        user.id,
        payload.content
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error adding note: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(note))
}

/// GET /notes - Fetch all notes with author_email
pub async fn get_notes(
    State(state): State<SharedState>,
) -> Result<Json<Vec<NoteWithAuthor>>, StatusCode> {
    let notes = query_as_unchecked!(
        NoteWithAuthor,
        r#"
        SELECT 
            n.id, 
            n.ticket_id, 
            n.author_id, 
            n.content, 
            n.created_at, 
            u.email AS author_email
        FROM notes n
        JOIN users u ON u.id = n.author_id
        ORDER BY n.created_at ASC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error fetching notes: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(notes))
}

/// POST /comments - Add a comment to a note
pub async fn add_comment(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
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
        user.id,
        payload.content
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error adding comment: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(comment))
}

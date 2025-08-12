use crate::models::note::{Note, NoteWithAuthor, CreateNoteInput};
use sqlx::{PgPool, query_as_unchecked};
use uuid::Uuid;
use chrono::Utc;

/// Create a new note
pub async fn create_note(
    pool: &PgPool,
    input: CreateNoteInput,
    author_id: Uuid,
) -> Result<Note, sqlx::Error> {
    let note = query_as_unchecked!(
        Note,
        r#"
        INSERT INTO notes (id, ticket_id, author_id, content, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, ticket_id, author_id, content, created_at
        "#,
        Uuid::new_v4(),
        input.ticket_id,
        author_id,
        input.content,
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(note)
}

/// Get notes with author email for a ticket
pub async fn get_notes_with_author_by_ticket_id(
    pool: &PgPool,
    ticket_id: Uuid,
) -> Result<Vec<NoteWithAuthor>, sqlx::Error> {
    let notes = query_as_unchecked!(
        NoteWithAuthor,
        r#"
        SELECT 
            notes.id,
            notes.ticket_id,
            notes.author_id,
            notes.content,
            notes.created_at,
            users.email as author_email
        FROM notes
        JOIN users ON notes.author_id = users.id
        WHERE notes.ticket_id = $1
        ORDER BY notes.created_at DESC
        "#,
        ticket_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notes)
}

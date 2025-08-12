use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    dto::kb_dto::{CreateArticleRequest, CreateCategoryRequest},
    models::knowledge_base::{KBArticle, KBCategory},
    services::knowledge_base_service,
    state::SharedState,
    utils::slug::to_slug,
};

// Create a new category
pub async fn create_category(
    State(state): State<SharedState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<KBCategory>, StatusCode> {
    let category = sqlx::query_as_unchecked!(
        KBCategory,
        r#"
        INSERT INTO kb_categories (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at
        "#,
        payload.name,
        payload.description
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        tracing::error!("DB error creating category: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(category))
}

// Create a new article
pub async fn create_article(
    State(state): State<SharedState>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<KBArticle>, StatusCode> {
    let slug = to_slug(&payload.title);
    let now = chrono::Utc::now();
    let article_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO kb_articles (
            id, category_id, title, slug, content, author_id, is_published,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        article_id,
        payload.category_id,
        payload.title,
        slug,
        payload.content,
        payload.author_id,
        payload.is_published.unwrap_or(false),
        now,
        now
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("DB error inserting article: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut tag_names: Vec<String> = vec![];

    if let Some(tags) = &payload.tags {
        for tag in tags {
            tag_names.push(tag.clone());

            let tag_id = sqlx::query_scalar!(
                r#"
                INSERT INTO tags (name) VALUES ($1)
                ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
                RETURNING id
                "#,
                tag
            )
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Tag insert error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            sqlx::query!(
                r#"
                INSERT INTO article_tags (article_id, tag_id)
                VALUES ($1, $2) ON CONFLICT DO NOTHING
                "#,
                article_id,
                tag_id
            )
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Tag-article link error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
    }

    let article = KBArticle {
        id: article_id,
        category_id: payload.category_id,
        title: payload.title.clone(),
        slug,
        content: payload.content.clone(),
        author_id: payload.author_id,
        is_published: payload.is_published.unwrap_or(false),
        created_at: now,
        updated_at: Some(now),
        tags: if tag_names.is_empty() { None } else { Some(tag_names) },
    };

    Ok(Json(article))
}

// Get all categories
pub async fn get_all_categories(
    State(state): State<SharedState>,
) -> Result<Json<Vec<KBCategory>>, StatusCode> {
    let categories = sqlx::query_as!(
        KBCategory,
        r#"
        SELECT id, name, description, created_at
        FROM kb_categories
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Category fetch error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(categories))
}

// Get all articles
pub async fn get_all_articles(
    State(state): State<SharedState>,
) -> Result<Json<Vec<KBArticle>>, StatusCode> {
    let articles = sqlx::query_as!(
        KBArticle,
        r#"
        SELECT a.id, a.category_id, a.title, a.slug, a.content,
               a.author_id, a.is_published, a.created_at, a.updated_at,
               ARRAY(
                   SELECT t.name FROM tags t
                   JOIN article_tags at ON t.id = at.tag_id
                   WHERE at.article_id = a.id
               ) AS "tags!"
        FROM kb_articles a
        ORDER BY a.created_at DESC
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Article fetch error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(articles))
}

// Get article by ID
pub async fn get_article_by_id(
    State(state): State<SharedState>,
    Path(article_id): Path<Uuid>,
) -> Result<Json<KBArticle>, StatusCode> {
    let article = sqlx::query_as!(
        KBArticle,
        r#"
        SELECT a.id, a.category_id, a.title, a.slug, a.content,
               a.author_id, a.is_published, a.created_at, a.updated_at,
               ARRAY(
                   SELECT t.name FROM tags t
                   JOIN article_tags at ON t.id = at.tag_id
                   WHERE at.article_id = a.id
               ) AS "tags!"
        FROM kb_articles a
        WHERE a.id = $1
        "#,
        article_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Fetch by ID error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(article))
}

// Get articles by category
pub async fn get_articles_by_category(
    State(state): State<SharedState>,
    Path(category_id): Path<Uuid>,
) -> Result<Json<Vec<KBArticle>>, StatusCode> {
    let articles = sqlx::query_as!(
        KBArticle,
        r#"
        SELECT a.id, a.category_id, a.title, a.slug, a.content,
               a.author_id, a.is_published, a.created_at, a.updated_at,
               ARRAY(
                   SELECT t.name FROM tags t
                   JOIN article_tags at ON t.id = at.tag_id
                   WHERE at.article_id = a.id
               ) AS "tags!"
        FROM kb_articles a
        WHERE a.category_id = $1
        ORDER BY a.created_at DESC
        "#,
        category_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Category fetch error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(articles))
}

// Get articles by tag
pub async fn get_articles_by_tag(
    Path(tag): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    match knowledge_base_service::get_articles_by_tag(&state.db, &tag).await {
        Ok(articles) => Json(articles).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to fetch articles by tag",
        )
            .into_response(),
    }
}

// Update article
pub async fn update_article_by_id(
    State(state): State<SharedState>,
    Path(article_id): Path<Uuid>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<Json<KBArticle>, StatusCode> {
    let slug = to_slug(&payload.title);
    let now = chrono::Utc::now();

    sqlx::query!(
        r#"
        UPDATE kb_articles
        SET category_id = $1,
            title = $2,
            slug = $3,
            content = $4,
            is_published = $5,
            updated_at = $6
        WHERE id = $7
        "#,
        payload.category_id,
        payload.title,
        slug,
        payload.content,
        payload.is_published.unwrap_or(false),
        now,
        article_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Update article error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query!(
        r#"
        DELETE FROM article_tags WHERE article_id = $1
        "#,
        article_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Delete article_tags error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut tag_names: Vec<String> = vec![];

    if let Some(tags) = &payload.tags {
        for tag in tags {
            tag_names.push(tag.clone());

            let tag_id = sqlx::query_scalar!(
                r#"
                INSERT INTO tags (name) VALUES ($1)
                ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
                RETURNING id
                "#,
                tag
            )
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Insert tag error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            sqlx::query!(
                r#"
                INSERT INTO article_tags (article_id, tag_id)
                VALUES ($1, $2) ON CONFLICT DO NOTHING
                "#,
                article_id,
                tag_id
            )
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Insert article_tag error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
    }

    get_article_by_id(State(state), Path(article_id)).await
}

// Delete article
pub async fn delete_article_by_id(
    State(state): State<SharedState>,
    Path(article_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query!(
        r#"
        DELETE FROM article_tags WHERE article_id = $1
        "#,
        article_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete article tags: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result = sqlx::query!(
        r#"
        DELETE FROM kb_articles WHERE id = $1
        "#,
        article_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to delete article: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok((StatusCode::NO_CONTENT, "Article deleted"))
}

// ✅ New: Get all tags (for autocomplete, filters, etc.)
pub async fn get_all_tags(
    State(state): State<SharedState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let tags = sqlx::query_scalar!(
        r#"
        SELECT name FROM tags ORDER BY name
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Error fetching tags: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(tags))
}

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::knowledge_base::{KBArticle, KBCategory};

#[derive(Debug, thiserror::Error)]
pub enum KnowledgeBaseError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Article not found")]
    ArticleNotFound,
    #[error("Category not found")]
    CategoryNotFound,
    #[error("Tag not found")]
    TagNotFound,
}

pub type Result<T> = std::result::Result<T, KnowledgeBaseError>;

pub async fn create_category(
    pool: &PgPool,
    name: String,
    description: Option<String>,
) -> Result<KBCategory> {
    let category = sqlx::query_as_unchecked!(
        KBCategory,
        r#"
        INSERT INTO kb_categories (name, description)
        VALUES ($1, $2)
        RETURNING id, name, description, created_at
        "#,
        name,
        description
    )
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn create_article(
    pool: &PgPool,
    category_id: Uuid,
    title: String,
    slug: String,
    content: String,
    author_id: Uuid,
    is_published: bool,
    tags: Option<Vec<String>>,
) -> Result<KBArticle> {
    let now = Utc::now();
    let article_id = Uuid::new_v4();

    // Start transaction
    let mut tx = pool.begin().await?;

    // Step 1: Insert article (without tags)
    sqlx::query!(
        r#"
        INSERT INTO kb_articles (
            id, category_id, title, slug, content, author_id, is_published,
            created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        article_id,
        category_id,
        title,
        slug,
        content,
        author_id,
        is_published,
        now,
        now
    )
    .execute(&mut *tx)
    .await?;

    // Step 2: Handle tags
    let mut tag_names: Vec<String> = vec![];

    if let Some(tags) = &tags {
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
            .fetch_one(&mut *tx)
            .await?;

            sqlx::query!(
                r#"
                INSERT INTO article_tags (article_id, tag_id)
                VALUES ($1, $2) ON CONFLICT DO NOTHING
                "#,
                article_id,
                tag_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    // Commit transaction
    tx.commit().await?;

    // Step 3: Return constructed KBArticle
    let article = KBArticle {
        id: article_id,
        category_id,
        title: title.clone(),
        slug,
        content: content.clone(),
        author_id,
        is_published,
        created_at: now,
        updated_at: Some(now),
        tags: if tag_names.is_empty() { None } else { Some(tag_names) },
    };

    Ok(article)
}

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<KBCategory>> {
    let categories = sqlx::query_as!(
        KBCategory,
        r#"
        SELECT id, name, description, created_at
        FROM kb_categories
        ORDER BY name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

pub async fn get_all_articles(pool: &PgPool) -> Result<Vec<KBArticle>> {
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
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn get_article_by_id(pool: &PgPool, article_id: Uuid) -> Result<KBArticle> {
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
    .fetch_optional(pool)
    .await?
    .ok_or(KnowledgeBaseError::ArticleNotFound)?;

    Ok(article)
}

pub async fn get_articles_by_category(pool: &PgPool, category_id: Uuid) -> Result<Vec<KBArticle>> {
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
    .fetch_all(pool)
    .await?;

    Ok(articles)
}

pub async fn get_articles_by_tag(pool: &PgPool, tag: &str) -> Result<Vec<KBArticle>> {
    let articles = sqlx::query_as!(
        KBArticle,
        r#"
        SELECT DISTINCT a.id, a.category_id, a.title, a.slug, a.content,
               a.author_id, a.is_published, a.created_at, a.updated_at,
               ARRAY(
                   SELECT t2.name FROM tags t2
                   JOIN article_tags at2 ON t2.id = at2.tag_id
                   WHERE at2.article_id = a.id
               ) AS "tags!"
        FROM kb_articles a
        JOIN article_tags at ON a.id = at.article_id
        JOIN tags t ON t.id = at.tag_id
        WHERE t.name = $1
        ORDER BY a.created_at DESC
        "#,
        tag
    )
    .fetch_all(pool)
    .await?;

    Ok(articles)
}
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub category_id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub is_published: Option<bool>,
    pub tags: Option<Vec<String>>,
}

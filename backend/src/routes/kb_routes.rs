use axum::{
    routing::{get, post, put, delete},
    Router,
};

use crate::{
    handlers::knowledge_base_handler::{
        create_article,
        create_category,
        get_all_articles,
        get_article_by_id,
        get_all_categories,
        get_articles_by_category,
        get_articles_by_tag,
        update_article_by_id,
        delete_article_by_id,
        get_all_tags, // Make sure this exists
    },
    state::SharedState,
};

/// 🆓 Public KB routes — accessible without authentication
pub fn public_kb_routes(state: SharedState) -> Router {
    Router::new()
        .route("/kb/articles", get(get_all_articles))
        .route("/kb/articles/{id}", get(get_article_by_id))
        .route("/kb/categories", get(get_all_categories))
        .route("/kb/categories/{category_id}/articles", get(get_articles_by_category))
        .route("/kb/articles/tag/{tag}", get(get_articles_by_tag))
        .route("/kb/tags", get(get_all_tags)) // for tag filters/autocomplete
        .with_state(state)
}

/// 🔒 Protected KB routes — for admin/agent only
pub fn protected_kb_routes(state: SharedState) -> Router {
    Router::new()
        .route("/kb/articles", post(create_article))
        .route("/kb/articles/{id}", {
            put(update_article_by_id)
                .delete(delete_article_by_id)
        })
        .route("/kb/categories", post(create_category))
        .with_state(state)
}

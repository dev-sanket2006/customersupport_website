use axum::{Router, middleware};
use tower_http::cors::{Any, CorsLayer};
use tokio::sync::{RwLock, broadcast};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::AppConfig,
    db,
    middleware::{
        auth_extension::require_auth,
        rate_limit::rate_limit_middleware,
    },
    routes::*,
    state::{AppState, SharedState},
};
use crate::routes::{agent_ticket_routes, kb_routes::{public_kb_routes, protected_kb_routes}};

pub async fn create_app() -> anyhow::Result<Router> {
    let config = AppConfig::from_env();
    let pool = db::connect_to_db(&config.database_url).await;

    // ✅ Initialize WebSocket channels map
    let ws_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>> =
        Arc::new(RwLock::new(HashMap::new()));

    // ✅ Create Shared App State
    let shared_state = SharedState::new(AppState {
        db: pool,
        config: config.clone(),
        ws_channels,
    });

    // 🌍 Global CORS policy - allows all origins, methods, and headers
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 🔐 Protected routes with auth and rate limiting
    let protected_routes = Router::new()
        .merge(ticket_routes::routes(shared_state.clone()))
        .merge(note_routes::routes(shared_state.clone()))
        .merge(message_routes::routes(shared_state.clone()))
        .merge(attachment_routes::routes(shared_state.clone()))
        .merge(protected_kb_routes(shared_state.clone())) // ✅ Updated
        
        .merge(report_routes::routes(shared_state.clone()))
        .merge(user_routes::routes(shared_state.clone()))
        .merge(comment_routes::routes(shared_state.clone()))
        .merge(agent_ticket_routes::routes(shared_state.clone()))
        .layer(middleware::from_fn_with_state(shared_state.clone(), require_auth))
        .layer(middleware::from_fn(rate_limit_middleware));

    // 🆓 Public routes (no authentication)
    let public_routes = Router::new()
        .merge(auth_routes::routes(shared_state.clone()))
        .merge(ws_routes::routes(shared_state.clone())) // WebSocket has its own auth
        .merge(public_kb_routes(shared_state.clone())) // ✅ Updated
        .merge(notification_routes::routes(shared_state.clone()));
    // 🛠️ Compose final app
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors);

    Ok(app)
}

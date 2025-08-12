use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};
use uuid::Uuid;

use sqlx::PgPool;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
    pub ws_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>,
}

pub type SharedState = Arc<AppState>;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,                         // UUID from gen_random_uuid()
    pub name: String,                     // TEXT NOT NULL
    pub email: String,                    // TEXT NOT NULL UNIQUE
    pub password_hash: String,           // TEXT NOT NULL
    pub role: String,                    // TEXT NOT NULL DEFAULT 'user'
    pub is_active: bool,                 // BOOLEAN NOT NULL DEFAULT TRUE
    pub created_at: Option<DateTime<Utc>>, // ✅ Made optional to allow NULLs
    pub updated_at: Option<DateTime<Utc>>, // ✅ Made optional to allow NULLs
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub is_active: Option<bool>,              // <--- changed
    pub created_at: Option<DateTime<Utc>>,    // if needed
    pub updated_at: Option<DateTime<Utc>>,    // if needed
}
#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
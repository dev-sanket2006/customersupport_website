use crate::models::user::{RegisterInput, LoginInput, User};
use crate::utils::jwt::generate_jwt;
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use anyhow::{Result, anyhow};

pub async fn register_user(pool: &PgPool, input: RegisterInput) -> Result<User> {
    let hashed = hash(&input.password, DEFAULT_COST)?;

    let row = sqlx::query_as::<_, User>(r#"
        INSERT INTO users (id, name, email, password_hash, role)
        VALUES ($1, $2, $3, $4, 'agent')
        RETURNING id, name, email, password_hash, role, created_at
    "#)
    .bind(Uuid::new_v4())
    .bind(&input.name)
    .bind(&input.email)
    .bind(&hashed)
    .fetch_one(pool)
    .await?;

    Ok(row) // ✅ return the inserted user
}

// ✅ Updated to accept jwt_secret parameter
pub async fn login_user(pool: &PgPool, input: LoginInput, jwt_secret: &str) -> Result<String> {
    let user = sqlx::query_as::<_, User>(r#"
        SELECT id, name, email, password_hash, role, created_at
        FROM users
        WHERE email = $1
    "#)
    .bind(&input.email)
    .fetch_optional(pool)
    .await?;

    let user = user.ok_or_else(|| anyhow!("Invalid email or password"))?;

    if !verify(&input.password, &user.password_hash)? {
        return Err(anyhow!("Invalid email or password"));
    }

    // ✅ Pass the jwt_secret to generate_jwt
    let token = generate_jwt(&user, jwt_secret)?;
    Ok(token)
}
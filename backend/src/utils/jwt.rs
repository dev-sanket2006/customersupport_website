use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::models::user::User;
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,         // user ID
    pub email: String,
    pub role: String,      // user/agent/admin
    pub exp: usize,        // expiration timestamp
    pub iat: usize,        // issued at timestamp
}

/// Used during auth service - NOW TAKES SECRET AS PARAMETER
pub fn generate_jwt(user: &User, secret: &str) -> Result<String> {
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(Duration::days(7))
        .ok_or_else(|| anyhow!("invalid timestamp"))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        role: user.role.clone(),
        iat: now.timestamp() as usize,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()), // ✅ Use provided secret
    )?;

    Ok(token)
}

/// Token creator for custom use
pub fn create_token(
    user_id: Uuid,
    email: String,
    role: String,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expiration = now
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email,
        role,
        iat: now.timestamp() as usize,
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

/// Token decoder
pub fn decode_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    println!("🔓 Decoding token with secret: '{}'", secret); // Debug log
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    
    println!("✅ Token decoded successfully for user: {}", token_data.claims.sub);
    Ok(token_data.claims)
}
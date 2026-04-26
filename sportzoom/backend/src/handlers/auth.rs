use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::db::models::{Claims, RegisterRequest, LoginRequest, AuthResponse, UserPublic, UserRole};
use crate::middleware::auth::Claims as JwtClaims;
use crate::utils::errors::{AppError, Result};
use std::sync::Arc;

/// Register a new user
#[axum::debug_handler]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    payload.validate()?;

    // Check if username or email already exists
    let existing = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_one(&state.db)
    .await?;

    if existing {
        return Err(AppError::conflict("Username or email already exists"));
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    // Create user
    let user_id = Uuid::new_v4();
    let user = sqlx::query_as::<_, UserPublic>(
        r#"
        INSERT INTO users (id, username, email, password_hash, display_name, points, level, streak, role)
        VALUES ($1, $2, $3, $4, $5, 0, 1, 0, 'user')
        RETURNING id, username, display_name, avatar_url, bio, country, points, level
        "#
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.display_name)
    .fetch_one(&state.db)
    .await?;

    // Generate tokens
    let tokens = generate_tokens(user_id, &payload.username, &UserRole::User, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user,
    }))
}

/// Login user
#[axum::debug_handler]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    payload.validate()?;

    // Get user from database
    let user = sqlx::query_as::<_, crate::db::models::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::auth("Invalid email or password"))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::auth("Invalid password hash"))?;
    
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::auth("Invalid email or password"))?;

    // Update last active
    sqlx::query("UPDATE users SET last_active = NOW() WHERE id = $1")
        .bind(user.id)
        .execute(&state.db)
        .await?;

    // Generate tokens
    let tokens = generate_tokens(user.id, &user.username, &user.role, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        user: user.into(),
    }))
}

/// Refresh access token
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[axum::debug_handler]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>> {
    // Validate refresh token
    let claims = validate_refresh_token(&payload.refresh_token, &state.jwt_secret)?;

    // Generate new tokens
    let tokens = generate_tokens(claims.sub, &claims.username, &claims.role, &state.jwt_secret)?;

    Ok(Json(RefreshResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

/// Logout user (invalidate refresh token)
#[axum::debug_handler]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<JwtClaims>,
) -> Result<StatusCode> {
    // In production, add refresh token to blacklist in Redis
    let user_id = extensions.sub;
    
    // Log the logout event
    tracing::info!("User {} logged out", user_id);

    Ok(StatusCode::OK)
}

/// Token pair structure
#[derive(Debug, Serialize)]
struct Tokens {
    access_token: String,
    refresh_token: String,
}

/// Generate access and refresh tokens
fn generate_tokens(user_id: Uuid, username: &str, role: &UserRole, secret: &str) -> Result<Tokens> {
    let now = Utc::now();
    
    // Access token: 15 minutes
    let access_exp = now + Duration::minutes(15);
    let access_claims = Claims {
        sub: user_id,
        username: username.to_string(),
        role: role.clone(),
        exp: access_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    // Refresh token: 7 days
    let refresh_exp = now + Duration::days(7);
    let refresh_claims = Claims {
        sub: user_id,
        username: username.to_string(),
        role: role.clone(),
        exp: refresh_exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(Tokens {
        access_token,
        refresh_token,
    })
}

/// Validate refresh token
fn validate_refresh_token(token: &str, secret: &str) -> Result<Claims> {
    use jsonwebtoken::{decode, Validation};
    
    let key = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}

/// Get current user profile
#[axum::debug_handler]
pub async fn get_current_user(
    State(_state): State<Arc<AppState>>,
    extensions: axum::extract::Extension<JwtClaims>,
) -> Result<Json<UserPublic>> {
    // This would normally fetch from DB, but we have the claims
    // For full user data, you'd query the database
    Ok(Json(UserPublic {
        id: extensions.sub,
        username: extensions.username.clone(),
        display_name: None, // Would need to fetch from DB
        avatar_url: None,
        bio: None,
        country: None,
        points: 0,
        level: 1,
    }))
}

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::config::AppState;
use crate::db::models::UserRole;
use crate::utils::errors::{AppError, Result};

/// JWT Claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // User ID
    pub username: String,
    pub role: UserRole,
    pub exp: usize,  // Expiration time (seconds since epoch)
    pub iat: usize,  // Issued at (seconds since epoch)
}

/// Extract user ID from JWT token in Authorization header
pub async fn auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, (StatusCode, String)> {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err((StatusCode::UNAUTHORIZED, "Missing or invalid authorization header".to_string())),
    };

    let claims = validate_token(token, &state.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    // Insert user info into request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Validate JWT token and extract claims
fn validate_token(token: &str, secret: &str) -> std::result::Result<Claims, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}

/// Optional authentication - doesn't fail if no token is present
pub async fn optional_auth_middleware<B>(
    State(state): State<Arc<AppState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = &header[7..];
            if let Ok(claims) = validate_token(token, &state.jwt_secret) {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

/// Admin-only middleware
pub async fn admin_middleware<B>(
    State(_state): State<Arc<AppState>>,
    request: Request<B>,
    next: Next<B>,
) -> std::result::Result<Response, (StatusCode, String)> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Authentication required".to_string()))?;

    if claims.role != UserRole::Admin && claims.role != UserRole::Moderator {
        return Err((StatusCode::FORBIDDEN, "Admin access required".to_string()));
    }

    Ok(next.run(request).await)
}

/// Extension trait to extract claims from request
pub trait ClaimsExt {
    fn get_claims(&self) -> Option<&Claims>;
    fn get_user_id(&self) -> Option<Uuid>;
}

impl<B> ClaimsExt for Request<B> {
    fn get_claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }

    fn get_user_id(&self) -> Option<Uuid> {
        self.get_claims().map(|c| c.sub)
    }
}

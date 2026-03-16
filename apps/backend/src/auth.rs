use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static JWT_SECRET: OnceLock<Vec<u8>> = OnceLock::new();

/// Call at startup to ensure JWT secret is set (panics in production if unset).
pub fn ensure_jwt_secret() {
    let _ = get_jwt_secret();
}

fn get_jwt_secret() -> &'static [u8] {
    JWT_SECRET.get_or_init(|| {
        let secret = std::env::var("JWT_SECRET")
            .or_else(|_| std::env::var("PMS_JWT_SECRET"))
            .unwrap_or_else(|_| {
                let prod =
                    std::env::var("ENVIRONMENT").as_ref().map(|s| s.as_str()) == Ok("production");
                if prod {
                    panic!("JWT_SECRET or PMS_JWT_SECRET must be set when ENVIRONMENT=production");
                }
                "pms-secret-change-in-production".to_string()
            });
        secret.into_bytes()
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    #[serde(default = "default_role")]
    pub role: String,
    pub exp: usize,
    pub iat: usize,
    /// When true, token is only valid for POST /auth/2fa/verify.
    #[serde(default)]
    pub two_fa_pending: bool,
}

fn default_role() -> String {
    "student".to_string()
}

pub fn create_token(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::days(30)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
        iat: now.timestamp() as usize,
        two_fa_pending: false,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret()),
    )
}

/// Short-lived token (5 min) only valid for POST /auth/2fa/verify.
pub fn create_token_2fa_pending(
    user_id: &str,
    role: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::minutes(5)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
        iat: now.timestamp() as usize,
        two_fa_pending: true,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_jwt_secret()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_jwt_secret()),
        &Validation::default(),
    )
    .map(|d| d.claims)
}

pub struct AuthUser(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(auth) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid authorization header",
                    )
                })?;
        let claims = verify_token(auth.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;
        if claims.two_fa_pending {
            return Err((StatusCode::UNAUTHORIZED, "2FA verification required"));
        }
        Ok(AuthUser(claims.sub))
    }
}

/// Extractor for routes that require admin or super_admin role (from JWT).
pub struct AdminUser(pub String, pub String); // (user_id, role)

#[async_trait]
impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(auth) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid authorization header",
                    )
                })?;
        let claims = verify_token(auth.token())
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))?;
        if claims.two_fa_pending {
            return Err((StatusCode::UNAUTHORIZED, "2FA verification required"));
        }
        let role = claims.role.as_str();
        if role != "admin" && role != "super_admin" {
            return Err((StatusCode::FORBIDDEN, "Admin or super_admin role required"));
        }
        Ok(AdminUser(claims.sub, claims.role))
    }
}

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"pms-secret-change-in-production"; // TODO: use env var

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    #[serde(default = "default_role")]
    pub role: String,
    pub exp: usize,
    pub iat: usize,
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
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
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
        let role = claims.role.as_str();
        if role != "admin" && role != "super_admin" {
            return Err((
                StatusCode::FORBIDDEN,
                "Admin or super_admin role required",
            ));
        }
        Ok(AdminUser(claims.sub, claims.role))
    }
}

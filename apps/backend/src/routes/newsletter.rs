//! Newsletter: visitor subscription (email only).

use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use serde::Deserialize;
use sqlx::AnyPool;
use uuid::Uuid;

use crate::db;

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
}

pub fn newsletter_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/newsletter/subscribe", post(subscribe))
        .with_state(pool)
}

async fn subscribe(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<SubscribeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email = req.email.trim();
    if email.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "email_required",
                "message": "Email is required"
            })),
        ));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let result = if db::is_postgres() {
        sqlx::query(
            "INSERT INTO newsletter_subscribers (id, email, created_at) VALUES ($1, $2, $3) \
             ON CONFLICT (email) DO NOTHING",
        )
        .bind(&id)
        .bind(email)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        // SQLite: INSERT OR IGNORE for unique email
        sqlx::query(
            "INSERT OR IGNORE INTO newsletter_subscribers (id, email, created_at) VALUES (?, ?, ?)",
        )
        .bind(&id)
        .bind(email)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Newsletter subscribe error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "subscribe_failed",
                "message": "Could not subscribe"
            })),
        )
    })?;

    let subscribed = result.rows_affected() > 0;
    Ok(Json(serde_json::json!({
        "subscribed": true,
        "message": if subscribed { "Thank you for subscribing." } else { "You're already subscribed." }
    })))
}

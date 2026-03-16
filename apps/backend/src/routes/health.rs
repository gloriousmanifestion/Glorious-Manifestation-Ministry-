//! Health check endpoint with optional database probe.

use axum::{extract::State, response::Json, routing::get, Router};
use serde::Serialize;
use sqlx::AnyPool;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    pub version: &'static str,
    pub timestamp: String,
}

pub fn health_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .with_state(pool)
}

async fn health_handler(State(pool): State<AnyPool>) -> Json<HealthResponse> {
    let database = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => Some("ok".to_string()),
        Err(e) => {
            tracing::warn!("Health check DB error: {:?}", e);
            Some("error".to_string())
        }
    };
    let status = if database.as_deref() == Some("ok") {
        "ok"
    } else {
        "degraded"
    };
    Json(HealthResponse {
        status: status.to_string(),
        database,
        version: env!("CARGO_PKG_VERSION"),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

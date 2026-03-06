use axum::{extract::State, response::Json, routing::get, Router};
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EventRow {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

pub fn events_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/events", get(list_events))
        .with_state(pool)
}

async fn list_events(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<EventRow>>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let events: Vec<EventRow> =
        sqlx::query_as("SELECT id, title, date, description FROM events ORDER BY date ASC")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!("List events error: {:?}", e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "fetch_failed",
                        "message": "Could not fetch events"
                    })),
                )
            })?;
    Ok(Json(events))
}

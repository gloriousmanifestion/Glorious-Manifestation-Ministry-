//! Resource library: public list of resources (optional filter by course).

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;

use crate::db;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ResourceRow {
    pub id: String,
    pub course_id: Option<String>,
    pub title: String,
    pub url: String,
    pub resource_type: String,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListResourcesQuery {
    pub course_id: Option<String>,
}

pub fn resources_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/resources", get(list_resources))
        .with_state(pool)
}

async fn list_resources(
    State(pool): State<AnyPool>,
    Query(q): Query<ListResourcesQuery>,
) -> Result<Json<Vec<ResourceRow>>, (StatusCode, Json<serde_json::Value>)> {
    let resources: Vec<ResourceRow> = if let Some(ref course_id) = q.course_id {
        if db::is_postgres() {
            sqlx::query_as(
                "SELECT id, course_id, title, url, resource_type, sort_order, created_at \
                 FROM resources WHERE course_id = $1 OR course_id IS NULL \
                 ORDER BY sort_order ASC, created_at ASC",
            )
            .bind(course_id)
            .fetch_all(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT id, course_id, title, url, resource_type, sort_order, created_at \
                 FROM resources WHERE course_id = ? OR course_id IS NULL \
                 ORDER BY sort_order ASC, created_at ASC",
            )
            .bind(course_id)
            .fetch_all(&pool)
            .await
        }
    } else {
        sqlx::query_as(
            "SELECT id, course_id, title, url, resource_type, sort_order, created_at \
             FROM resources ORDER BY sort_order ASC, created_at ASC",
        )
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("List resources error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load resources"})),
        )
    })?;
    Ok(Json(resources))
}

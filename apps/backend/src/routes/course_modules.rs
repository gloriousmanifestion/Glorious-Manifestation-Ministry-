//! Course curriculum / syllabus: public list of modules per course.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use sqlx::AnyPool;

use crate::db;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CourseModuleRow {
    pub id: String,
    pub course_id: String,
    pub title: String,
    pub sort_order: i32,
    pub link: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

pub fn course_modules_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/courses/:course_id/modules", get(list_by_course))
        .with_state(pool)
}

async fn list_by_course(
    State(pool): State<AnyPool>,
    Path(course_id): Path<String>,
) -> Result<Json<Vec<CourseModuleRow>>, (StatusCode, Json<serde_json::Value>)> {
    let modules: Vec<CourseModuleRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, course_id, title, sort_order, link, description, created_at FROM course_modules \
             WHERE course_id = $1 ORDER BY sort_order ASC, created_at ASC",
        )
        .bind(&course_id)
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, course_id, title, sort_order, link, description, created_at FROM course_modules \
             WHERE course_id = ? ORDER BY sort_order ASC, created_at ASC",
        )
        .bind(&course_id)
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("List course modules error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load curriculum"})),
        )
    })?;
    Ok(Json(modules))
}

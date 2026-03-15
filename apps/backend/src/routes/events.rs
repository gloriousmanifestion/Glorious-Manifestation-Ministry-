//! Events: public list + admin CRUD.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, patch},
    Router,
};
use serde::Deserialize;
use sqlx::AnyPool;
use uuid::Uuid;

use crate::auth::AdminUser;
use crate::db;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct EventRow {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

async fn list_events(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<EventRow>>, (StatusCode, Json<serde_json::Value>)> {
    let events: Vec<EventRow> =
        sqlx::query_as("SELECT id, title, date, description FROM events ORDER BY date ASC")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!("List events error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "fetch_failed",
                        "message": "Could not fetch events"
                    })),
                )
            })?;
    Ok(Json(events))
}

async fn list_admin(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
) -> Result<Json<Vec<EventRow>>, (StatusCode, Json<serde_json::Value>)> {
    let events: Vec<EventRow> =
        sqlx::query_as("SELECT id, title, date, description FROM events ORDER BY date ASC")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Admin list events error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"message": "Could not fetch events"})),
                )
            })?;
    Ok(Json(events))
}

#[derive(Debug, Deserialize)]
pub struct CreateEventBody {
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

async fn create(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    axum::Json(body): axum::Json<CreateEventBody>,
) -> Result<Json<EventRow>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let title = body.title.trim();
    let date = body.date.trim();
    let description = body
        .description
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Title is required"})),
        ));
    }
    if date.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Date is required"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query("INSERT INTO events (id, title, date, description) VALUES ($1, $2, $3, $4)")
            .bind(&id)
            .bind(title)
            .bind(date)
            .bind(description)
            .execute(&pool)
            .await
    } else {
        sqlx::query("INSERT INTO events (id, title, date, description) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(title)
            .bind(date)
            .bind(description)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Create event error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not create event"})),
        )
    })?;

    Ok(Json(EventRow {
        id: id.clone(),
        title: title.to_string(),
        date: date.to_string(),
        description: description.map(String::from),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventBody {
    pub title: Option<String>,
    pub date: Option<String>,
    pub description: Option<String>,
}

async fn update(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateEventBody>,
) -> Result<Json<EventRow>, (StatusCode, Json<serde_json::Value>)> {
    let existing: Option<(String, String, Option<String>)> = if db::is_postgres() {
        sqlx::query_as("SELECT title, date, description FROM events WHERE id = $1")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_as("SELECT title, date, description FROM events WHERE id = ?")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Fetch event error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let (title, date, description): (String, String, Option<String>) = match existing {
        Some((t, d, desc)) => (
            body.title.as_deref().unwrap_or(&t).trim().to_string(),
            body.date.as_deref().unwrap_or(&d).trim().to_string(),
            body.description
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(String::from)
                .or(desc),
        ),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "Event not found"})),
            ));
        }
    };

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Title is required"})),
        ));
    }
    if date.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Date is required"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query("UPDATE events SET title = $1, date = $2, description = $3 WHERE id = $4")
            .bind(&title)
            .bind(&date)
            .bind(description.as_deref())
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("UPDATE events SET title = ?, date = ?, description = ? WHERE id = ?")
            .bind(&title)
            .bind(&date)
            .bind(description.as_deref())
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Update event error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    Ok(Json(EventRow {
        id,
        title,
        date,
        description,
    }))
}

async fn delete_one(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM events WHERE id = $1")
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM events WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Delete event error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not delete"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Event not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn events_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/events", get(list_events))
        .route("/admin/events", get(list_admin).post(create))
        .route("/admin/events/:id", patch(update).delete(delete_one))
        .with_state(pool)
}

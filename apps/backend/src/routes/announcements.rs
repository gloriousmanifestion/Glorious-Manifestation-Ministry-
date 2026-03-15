//! Announcements: public list + admin CRUD.

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
pub struct AnnouncementRow {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created_at: String,
}

async fn list_public(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<AnnouncementRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<AnnouncementRow> = sqlx::query_as(
        "SELECT id, title, body, created_at FROM announcements ORDER BY created_at DESC LIMIT 50",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("List announcements error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch announcements"})),
        )
    })?;
    Ok(Json(rows))
}

async fn list_admin(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
) -> Result<Json<Vec<AnnouncementRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<AnnouncementRow> = sqlx::query_as(
        "SELECT id, title, body, created_at FROM announcements ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin list announcements error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch announcements"})),
        )
    })?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct CreateAnnouncementBody {
    pub title: String,
    pub body: String,
}

async fn create(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    axum::Json(body): axum::Json<CreateAnnouncementBody>,
) -> Result<Json<AnnouncementRow>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let title = body.title.trim();
    let body_text = body.body.trim();
    let now = chrono::Utc::now().to_rfc3339();

    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO announcements (id, title, body, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(&id)
        .bind(title)
        .bind(body_text)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query("INSERT INTO announcements (id, title, body, created_at) VALUES (?, ?, ?, ?)")
            .bind(&id)
            .bind(title)
            .bind(body_text)
            .bind(&now)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Create announcement error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not create announcement"})),
        )
    })?;

    let row = AnnouncementRow {
        id: id.clone(),
        title: title.to_string(),
        body: body_text.to_string(),
        created_at: now.clone(),
    };
    Ok(Json(row))
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnnouncementBody {
    pub title: Option<String>,
    pub body: Option<String>,
}

async fn update(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateAnnouncementBody>,
) -> Result<Json<AnnouncementRow>, (StatusCode, Json<serde_json::Value>)> {
    let existing: Option<(String, String)> = if db::is_postgres() {
        sqlx::query_as("SELECT title, body FROM announcements WHERE id = $1")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_as("SELECT title, body FROM announcements WHERE id = ?")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Fetch announcement error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let (title, body_text) = match existing {
        Some((t, b)) => (
            body.title.as_deref().unwrap_or(&t).trim().to_string(),
            body.body.as_deref().unwrap_or(&b).trim().to_string(),
        ),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "Announcement not found"})),
            ));
        }
    };

    if db::is_postgres() {
        sqlx::query("UPDATE announcements SET title = $1, body = $2 WHERE id = $3")
            .bind(&title)
            .bind(&body_text)
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("UPDATE announcements SET title = ?, body = ? WHERE id = ?")
            .bind(&title)
            .bind(&body_text)
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Update announcement error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let created_at: String = if db::is_postgres() {
        sqlx::query_scalar("SELECT created_at FROM announcements WHERE id = $1")
            .bind(&id)
            .fetch_one(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT created_at FROM announcements WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
    }
    .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());

    Ok(Json(AnnouncementRow {
        id,
        title,
        body: body_text,
        created_at,
    }))
}

async fn delete_one(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM announcements WHERE id = $1")
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM announcements WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Delete announcement error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not delete"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Announcement not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn announcements_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/announcements", get(list_public))
        .route("/admin/announcements", get(list_admin).post(create))
        .route("/admin/announcements/:id", patch(update).delete(delete_one))
        .with_state(pool)
}

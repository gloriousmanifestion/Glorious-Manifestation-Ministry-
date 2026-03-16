//! Events: public list + admin CRUD + RSVP.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, patch, post},
    Router,
};
use serde::Deserialize;
use sqlx::AnyPool;
use uuid::Uuid;

use crate::auth::{AdminUser, AuthUser};
use crate::db;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct EventRow {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
}

/// Event list item with RSVP count (public list).
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct EventListRow {
    pub id: String,
    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub rsvp_count: i64,
}

async fn list_events(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<EventListRow>>, (StatusCode, Json<serde_json::Value>)> {
    let sql = if db::is_postgres() {
        "SELECT e.id, e.title, e.date, e.description, \
         COALESCE((SELECT COUNT(*)::bigint FROM event_rsvps r WHERE r.event_id = e.id), 0) AS rsvp_count \
         FROM events e ORDER BY e.date ASC"
    } else {
        "SELECT e.id, e.title, e.date, e.description, \
         COALESCE((SELECT COUNT(*) FROM event_rsvps r WHERE r.event_id = e.id), 0) AS rsvp_count \
         FROM events e ORDER BY e.date ASC"
    };
    let events: Vec<EventListRow> = sqlx::query_as(sql).fetch_all(&pool).await.map_err(|e| {
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

/// Return event IDs the current user has RSVP'd to.
async fn get_my_rsvps(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<String>>, (StatusCode, Json<serde_json::Value>)> {
    let sql = if db::is_postgres() {
        "SELECT event_id FROM event_rsvps WHERE user_id = $1 ORDER BY created_at DESC"
    } else {
        "SELECT event_id FROM event_rsvps WHERE user_id = ? ORDER BY created_at DESC"
    };
    let rows: Vec<(String,)> = sqlx::query_as(sql)
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Get my RSVPs error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Could not fetch RSVPs"})),
            )
        })?;
    Ok(Json(rows.into_iter().map(|r| r.0).collect()))
}

async fn add_rsvp(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path(event_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Check event exists
    let exists = if db::is_postgres() {
        sqlx::query_scalar::<_, i64>("SELECT 1 FROM events WHERE id = $1")
            .bind(&event_id)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_scalar::<_, i64>("SELECT 1 FROM events WHERE id = ?")
            .bind(&event_id)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Check event error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not add RSVP"})),
        )
    })?;

    if exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Event not found"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO event_rsvps (event_id, user_id) VALUES ($1, $2) ON CONFLICT (event_id, user_id) DO NOTHING",
        )
        .bind(&event_id)
        .bind(&user_id)
        .execute(&pool)
        .await
    } else {
        sqlx::query("INSERT OR IGNORE INTO event_rsvps (event_id, user_id) VALUES (?, ?)")
            .bind(&event_id)
            .bind(&user_id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Add RSVP error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not add RSVP"})),
        )
    })?;

    Ok(StatusCode::NO_CONTENT)
}

async fn remove_rsvp(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path(event_id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM event_rsvps WHERE event_id = $1 AND user_id = $2")
            .bind(&event_id)
            .bind(&user_id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM event_rsvps WHERE event_id = ? AND user_id = ?")
            .bind(&event_id)
            .bind(&user_id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Remove RSVP error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not remove RSVP"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "RSVP not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
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
        .route("/events/:id/rsvp", post(add_rsvp).delete(remove_rsvp))
        .route("/me/event-rsvps", get(get_my_rsvps))
        .route("/admin/events", get(list_admin).post(create))
        .route("/admin/events/:id", patch(update).delete(delete_one))
        .with_state(pool)
}

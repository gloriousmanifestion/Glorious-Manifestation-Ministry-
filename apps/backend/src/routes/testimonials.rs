//! Testimonials: public list + admin CRUD.

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
pub struct TestimonialRow {
    pub id: String,
    pub author_name: String,
    pub role_or_title: Option<String>,
    pub body: String,
    pub image_url: Option<String>,
    pub sort_order: i64,
    pub created_at: String,
}

async fn list_public(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<TestimonialRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<TestimonialRow> = sqlx::query_as(
        "SELECT id, author_name, role_or_title, body, image_url, sort_order, created_at FROM testimonials ORDER BY sort_order ASC, created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("List testimonials error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch testimonials"})),
        )
    })?;
    Ok(Json(rows))
}

async fn list_admin(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
) -> Result<Json<Vec<TestimonialRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<TestimonialRow> = sqlx::query_as(
        "SELECT id, author_name, role_or_title, body, image_url, sort_order, created_at FROM testimonials ORDER BY sort_order ASC, created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin list testimonials error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch testimonials"})),
        )
    })?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct CreateTestimonialBody {
    pub author_name: String,
    pub role_or_title: Option<String>,
    pub body: String,
    pub image_url: Option<String>,
    pub sort_order: Option<i64>,
}

async fn create(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    axum::Json(body): axum::Json<CreateTestimonialBody>,
) -> Result<Json<TestimonialRow>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let author_name = body.author_name.trim();
    let role_or_title = body
        .role_or_title
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from);
    let body_text = body.body.trim();
    let image_url = body
        .image_url
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from);
    let sort_order = body.sort_order.unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    if author_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Author name is required"})),
        ));
    }
    if body_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Body is required"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO testimonials (id, author_name, role_or_title, body, image_url, sort_order, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&id)
        .bind(author_name)
        .bind(role_or_title.as_deref())
        .bind(body_text)
        .bind(image_url.as_deref())
        .bind(sort_order)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO testimonials (id, author_name, role_or_title, body, image_url, sort_order, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(author_name)
        .bind(role_or_title.as_deref())
        .bind(body_text)
        .bind(image_url.as_deref())
        .bind(sort_order)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Create testimonial error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not create testimonial"})),
        )
    })?;

    Ok(Json(TestimonialRow {
        id: id.clone(),
        author_name: author_name.to_string(),
        role_or_title: role_or_title.clone(),
        body: body_text.to_string(),
        image_url: image_url.clone(),
        sort_order,
        created_at: now.clone(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateTestimonialBody {
    pub author_name: Option<String>,
    pub role_or_title: Option<String>,
    pub body: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: Option<i64>,
}

async fn update(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateTestimonialBody>,
) -> Result<Json<TestimonialRow>, (StatusCode, Json<serde_json::Value>)> {
    type TestimonialRowParts = (String, Option<String>, String, Option<String>, i64);
    let existing: Option<TestimonialRowParts> = if db::is_postgres() {
        sqlx::query_as("SELECT author_name, role_or_title, body, image_url, sort_order FROM testimonials WHERE id = $1")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_as("SELECT author_name, role_or_title, body, image_url, sort_order FROM testimonials WHERE id = ?")
            .bind(&id)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Fetch testimonial error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let (author_name, role_or_title, body_text, image_url, sort_order) = match existing {
        Some((a, r, b, img, s)) => (
            body.author_name.as_deref().unwrap_or(&a).trim().to_string(),
            body.role_or_title
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(String::from)
                .or(r),
            body.body.as_deref().unwrap_or(&b).trim().to_string(),
            body.image_url
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(String::from)
                .or(img),
            body.sort_order.unwrap_or(s),
        ),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "Testimonial not found"})),
            ));
        }
    };

    if author_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Author name is required"})),
        ));
    }
    if body_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Body is required"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query(
            "UPDATE testimonials SET author_name = $1, role_or_title = $2, body = $3, image_url = $4, sort_order = $5 WHERE id = $6",
        )
        .bind(&author_name)
        .bind(role_or_title.as_deref())
        .bind(&body_text)
        .bind(image_url.as_deref())
        .bind(sort_order)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE testimonials SET author_name = ?, role_or_title = ?, body = ?, image_url = ?, sort_order = ? WHERE id = ?",
        )
        .bind(&author_name)
        .bind(role_or_title.as_deref())
        .bind(&body_text)
        .bind(image_url.as_deref())
        .bind(sort_order)
        .bind(&id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Update testimonial error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let created_at: String = if db::is_postgres() {
        sqlx::query_scalar("SELECT created_at FROM testimonials WHERE id = $1")
            .bind(&id)
            .fetch_one(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT created_at FROM testimonials WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
    }
    .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339());

    Ok(Json(TestimonialRow {
        id,
        author_name,
        role_or_title,
        body: body_text,
        image_url,
        sort_order,
        created_at,
    }))
}

async fn delete_one(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM testimonials WHERE id = $1")
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM testimonials WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Delete testimonial error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not delete"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Testimonial not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn testimonials_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/testimonials", get(list_public))
        .route("/admin/testimonials", get(list_admin).post(create))
        .route("/admin/testimonials/:id", patch(update).delete(delete_one))
        .with_state(pool)
}

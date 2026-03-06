use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

pub fn contact_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/contact", post(submit_contact))
        .with_state(pool)
}

async fn submit_contact(
    State(pool): State<SqlitePool>,
    axum::Json(req): axum::Json<ContactRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if req.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "name_required",
                "message": "Name is required"
            })),
        ));
    }
    if req.email.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "email_required",
                "message": "Email is required"
            })),
        ));
    }
    if req.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "message_required",
                "message": "Message is required"
            })),
        ));
    }

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO contact_submissions (id, name, email, subject, message, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(req.name.trim())
    .bind(req.email.trim())
    .bind(req.subject.as_deref())
    .bind(req.message.trim())
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Contact submit error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "submit_failed",
                "message": "Could not submit your message"
            })),
        )
    })?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Thank you for your message. We'll get back to you soon."
    })))
}

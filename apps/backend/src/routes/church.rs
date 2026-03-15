//! Church services and site content: public API + admin CRUD and artwork upload.

use axum::{
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use std::path::PathBuf;
use uuid::Uuid;

use crate::auth::AdminUser;
use crate::db;

fn upload_base() -> PathBuf {
    std::env::var("UPLOAD_BASE_PATH")
        .unwrap_or_else(|_| "uploads".into())
        .into()
}

// ---------- Public: list church services ----------

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ChurchServiceRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub day_of_week: String,
    pub time: String,
    pub sort_order: i64,
    pub image_url: Option<String>,
    pub features: Option<String>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

async fn list_services_public(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<ChurchServiceRow> = sqlx::query_as(
        "SELECT id, name, description, day_of_week, time, sort_order, image_url, features, is_active, created_at, updated_at \
         FROM church_services WHERE is_active = 1 ORDER BY sort_order, day_of_week, time",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("List church services error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "fetch_failed", "message": "Could not fetch church services"})),
        )
    })?;

    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            let features: Option<Vec<String>> = r
                .features
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok());
            serde_json::json!({
                "id": r.id,
                "name": r.name,
                "description": r.description,
                "day_of_week": r.day_of_week,
                "time": r.time,
                "sort_order": r.sort_order,
                "image_url": r.image_url,
                "features": features.unwrap_or_default(),
                "created_at": r.created_at,
                "updated_at": r.updated_at,
            })
        })
        .collect();
    Ok(Json(list))
}

// ---------- Admin: list all (including inactive) ----------

async fn list_services_admin(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<ChurchServiceRow> = sqlx::query_as(
        "SELECT id, name, description, day_of_week, time, sort_order, image_url, features, is_active, created_at, updated_at \
         FROM church_services ORDER BY sort_order, day_of_week, time",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin list church services error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load church services"})),
        )
    })?;

    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            let features: Option<Vec<String>> = r
                .features
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok());
            serde_json::json!({
                "id": r.id,
                "name": r.name,
                "description": r.description,
                "day_of_week": r.day_of_week,
                "time": r.time,
                "sort_order": r.sort_order,
                "image_url": r.image_url,
                "features": features.unwrap_or_default(),
                "is_active": r.is_active != 0,
                "created_at": r.created_at,
                "updated_at": r.updated_at,
            })
        })
        .collect();
    Ok(Json(list))
}

#[derive(Debug, Deserialize)]
pub struct CreateChurchServiceBody {
    pub name: String,
    pub description: Option<String>,
    pub day_of_week: String,
    pub time: String,
    pub sort_order: Option<i32>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

async fn create_service(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    axum::Json(body): axum::Json<CreateChurchServiceBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let name = body.name.trim();
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Name is required"})),
        ));
    }
    let description = body.description.as_deref().unwrap_or("").to_string();
    let day_of_week = body.day_of_week.trim().to_string();
    let time = body.time.trim().to_string();
    let sort_order = body.sort_order.unwrap_or(0);
    let features_json = body
        .features
        .as_ref()
        .and_then(|f| serde_json::to_string(f).ok());
    let is_active = body.is_active.unwrap_or(true) as i32;

    let now = chrono::Utc::now().to_rfc3339();
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO church_services (id, name, description, day_of_week, time, sort_order, features, is_active, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(&id)
        .bind(name)
        .bind(&description)
        .bind(&day_of_week)
        .bind(&time)
        .bind(sort_order)
        .bind(&features_json)
        .bind(is_active)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO church_services (id, name, description, day_of_week, time, sort_order, features, is_active, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(name)
        .bind(&description)
        .bind(&day_of_week)
        .bind(&time)
        .bind(sort_order)
        .bind(&features_json)
        .bind(is_active)
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Create church service error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to create church service"})),
        )
    })?;

    Ok(Json(serde_json::json!({
        "id": id,
        "name": name,
        "description": description,
        "day_of_week": day_of_week,
        "time": time,
        "sort_order": sort_order,
        "image_url": null,
        "features": body.features.unwrap_or_default(),
        "is_active": is_active != 0,
        "created_at": now,
        "updated_at": now,
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateChurchServiceBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub day_of_week: Option<String>,
    pub time: Option<String>,
    pub sort_order: Option<i32>,
    pub image_url: Option<String>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

/// Row from church_services for update: name, description, day_of_week, time, sort_order, image_url, features, is_active.
type ChurchServiceUpdateRow = (
    String,
    String,
    String,
    String,
    i32,
    Option<String>,
    Option<String>,
    i32,
);

async fn update_service(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateChurchServiceBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let existing: Option<ChurchServiceUpdateRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT name, description, day_of_week, time, sort_order, image_url, features, is_active FROM church_services WHERE id = $1",
        )
        .bind(&id)
        .fetch_optional(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT name, description, day_of_week, time, sort_order, image_url, features, is_active FROM church_services WHERE id = ?",
        )
        .bind(&id)
        .fetch_optional(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Fetch church service error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update"})),
        )
    })?;

    let (name, description, day_of_week, time, sort_order, image_url, features_json, is_active) =
        match existing {
            Some(r) => r,
            None => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"message": "Church service not found"})),
                ));
            }
        };

    let name = body.name.as_deref().unwrap_or(&name).trim().to_string();
    let description = body
        .description
        .as_deref()
        .unwrap_or(&description)
        .to_string();
    let day_of_week = body
        .day_of_week
        .as_deref()
        .unwrap_or(&day_of_week)
        .to_string();
    let time = body.time.as_deref().unwrap_or(&time).to_string();
    let sort_order = body.sort_order.unwrap_or(sort_order);
    let image_url = body.image_url.as_ref().or(image_url.as_ref()).cloned();
    let features_json = body
        .features
        .as_ref()
        .and_then(|f| serde_json::to_string(f).ok())
        .or(features_json);
    let is_active = body.is_active.map(|b| b as i32).unwrap_or(is_active);

    let now = chrono::Utc::now().to_rfc3339();
    if db::is_postgres() {
        sqlx::query(
            "UPDATE church_services SET name = $1, description = $2, day_of_week = $3, time = $4, sort_order = $5, image_url = $6, features = $7, is_active = $8, updated_at = $9 WHERE id = $10",
        )
        .bind(&name)
        .bind(&description)
        .bind(&day_of_week)
        .bind(&time)
        .bind(sort_order)
        .bind(&image_url)
        .bind(&features_json)
        .bind(is_active)
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE church_services SET name = ?, description = ?, day_of_week = ?, time = ?, sort_order = ?, image_url = ?, features = ?, is_active = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&name)
        .bind(&description)
        .bind(&day_of_week)
        .bind(&time)
        .bind(sort_order)
        .bind(&image_url)
        .bind(&features_json)
        .bind(is_active)
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Update church service error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update church service"})),
        )
    })?;

    let features: Vec<String> = features_json
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default();
    Ok(Json(serde_json::json!({
        "id": id,
        "name": name,
        "description": description,
        "day_of_week": day_of_week,
        "time": time,
        "sort_order": sort_order,
        "image_url": image_url,
        "features": features,
        "is_active": is_active != 0,
        "updated_at": now,
    })))
}

async fn delete_service(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM church_services WHERE id = $1")
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM church_services WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Delete church service error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to delete church service"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Church service not found"})),
        ));
    }
    Ok(Json(serde_json::json!({"ok": true})))
}

// ---------- Admin: upload artwork ----------

fn safe_filename(name: &str) -> String {
    let ext = std::path::Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let safe_ext = ext
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();
    format!(
        "{}.{}",
        Uuid::new_v4(),
        if safe_ext.is_empty() {
            "jpg"
        } else {
            &safe_ext
        }
    )
}

async fn upload_artwork(
    State(_pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    use tokio::io::AsyncWriteExt;

    let mut file_path: Option<PathBuf> = None;
    let base = upload_base();
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid_multipart", "message": "Invalid upload"})),
        )
    })? {
        if field.name().unwrap_or("") == "file" {
            let file_name = field
                .file_name()
                .map(safe_filename)
                .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));
            let bytes = field.bytes().await.map_err(|e| {
                tracing::error!("Read upload error: {:?}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error": "upload_failed", "message": "Could not read file"})),
                )
            })?;

            let dir = base.join("church");
            tokio::fs::create_dir_all(&dir).await.map_err(|e| {
                tracing::error!("Create upload dir error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "upload_failed", "message": "Could not prepare upload directory"})),
                )
            })?;
            let path = dir.join(&file_name);
            let mut file = tokio::fs::File::create(&path).await.map_err(|e| {
                tracing::error!("Create file error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "upload_failed", "message": "Could not save file"})),
                )
            })?;
            file.write_all(&bytes).await.map_err(|e| {
                tracing::error!("Write file error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "upload_failed", "message": "Could not save file"})),
                )
            })?;
            file_path = Some(path);
        }
    }

    let path = file_path.ok_or((
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": "file_required", "message": "Please upload a file"})),
    ))?;

    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("image.jpg");
    Ok(Json(serde_json::json!({
        "image_url": file_name,
        "url": format!("/api/v1/uploads/church/{}", file_name),
        "filename": file_name,
    })))
}

// ---------- Serve uploaded church files ----------

async fn serve_upload(Path(filename): Path<String>) -> Response {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return (StatusCode::BAD_REQUEST, "Invalid filename").into_response();
    }
    let base = upload_base();
    let path = base.join("church").join(&filename);
    if !path.starts_with(&base) {
        return (StatusCode::FORBIDDEN, "Invalid path").into_response();
    }
    match tokio::fs::read(&path).await {
        Ok(data) => {
            let content_type = infer_content_type(&filename);
            ([(header::CONTENT_TYPE, content_type)], data).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

fn infer_content_type(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "image/jpeg"
    }
}

// ---------- Site content (church about text, etc.) ----------

async fn get_site_content_public(
    State(pool): State<AnyPool>,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if key.is_empty() || key.contains('.') || key.contains('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid_key"})),
        ));
    }
    let row: Option<(String, String)> = if db::is_postgres() {
        sqlx::query_as("SELECT key, value FROM site_content WHERE key = $1")
            .bind(&key)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_as("SELECT key, value FROM site_content WHERE key = ?")
            .bind(&key)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Get site content error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "fetch_failed"})),
        )
    })?;

    match row {
        Some((k, value)) => Ok(Json(serde_json::json!({ "key": k, "value": value }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "not_found"})),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct SetSiteContentBody {
    pub value: String,
}

async fn set_site_content(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(key): Path<String>,
    axum::Json(body): axum::Json<SetSiteContentBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if key.is_empty() || key.contains('.') || key.contains('/') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Invalid key"})),
        ));
    }
    let now = chrono::Utc::now().to_rfc3339();
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO site_content (key, value, updated_at) VALUES ($1, $2, $3) \
             ON CONFLICT (key) DO UPDATE SET value = $2, updated_at = $3",
        )
        .bind(&key)
        .bind(&body.value)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO site_content (key, value, updated_at) VALUES (?, ?, ?) \
             ON CONFLICT (key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        )
        .bind(&key)
        .bind(&body.value)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Set site content error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to save"})),
        )
    })?;
    Ok(Json(
        serde_json::json!({ "key": key, "value": body.value, "updated_at": now }),
    ))
}

// ---------- Router ----------

pub fn church_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/church/services", get(list_services_public))
        .route("/church/content/:key", get(get_site_content_public))
        .route("/uploads/church/:filename", get(serve_upload))
        .route(
            "/admin/church/services",
            get(list_services_admin).post(create_service),
        )
        .route("/admin/church/services/upload", post(upload_artwork))
        .route(
            "/admin/church/services/:id",
            patch(update_service).delete(delete_service),
        )
        .route("/admin/church/content/:key", post(set_site_content))
        .with_state(pool)
}

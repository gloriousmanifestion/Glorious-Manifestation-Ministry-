use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use uuid::Uuid;

use crate::auth::AdminUser;
use crate::db;

// ---------- Metrics (camelCase for frontend AdminMetrics) ----------
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminMetricsResponse {
    pub total_registrations: u64,
    pub verified_count: u64,
    pub paid_count: u64,
    pub pending_payment_count: u64,
    pub revenue_total: u64,
    pub currency: String,
    pub registrations_this_month: u64,
    pub completion_rate: f64,
}

async fn metrics(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
) -> Result<Json<AdminMetricsResponse>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Metrics total error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to load metrics"})),
            )
        })?;
    let verified: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE verified = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Metrics verified error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to load metrics"})),
            )
        })?;
    let paid: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE payment_complete = 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Metrics paid error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to load metrics"})),
            )
        })?;
    let total_u = total.0.max(0) as u64;
    let verified_u = verified.0.max(0) as u64;
    let paid_u = paid.0.max(0) as u64;
    let pending = total_u.saturating_sub(paid_u);
    let completion_rate = if total_u > 0 {
        (verified_u as f64 / total_u as f64) * 100.0
    } else {
        0.0
    };
    let (month_count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE created_at >= date('now', 'start of month')",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or((0,));
    Ok(Json(AdminMetricsResponse {
        total_registrations: total_u,
        verified_count: verified_u,
        paid_count: paid_u,
        pending_payment_count: pending,
        revenue_total: 0,
        currency: "NGN".to_string(),
        registrations_this_month: month_count.max(0) as u64,
        completion_rate,
    }))
}

// ---------- Users ----------
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub ministry_role: String,
    pub church_name: String,
    pub verified: bool,
    pub payment_complete: bool,
    pub selected_course_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminUsersQuery {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

async fn list_users(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Query(q): Query<AdminUsersQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = q.page.unwrap_or(1).max(1);
    let limit = q.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    let (count,): (i64,) = if let Some(ref s) = q.search {
        let like = format!("%{}%", s.trim());
        if db::is_postgres() {
            sqlx::query_as(
                "SELECT COUNT(*) FROM users WHERE email LIKE $1 OR name LIKE $1 OR church_name LIKE $1",
            )
            .bind(&like)
            .fetch_one(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT COUNT(*) FROM users WHERE email LIKE ? OR name LIKE ? OR church_name LIKE ?",
            )
            .bind(&like)
            .bind(&like)
            .bind(&like)
            .fetch_one(&pool)
            .await
        }
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Admin list users count error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load users"})),
        )
    })?;

    let users: Vec<AdminUserRow> = if let Some(ref s) = q.search {
        let like = format!("%{}%", s.trim());
        if db::is_postgres() {
            sqlx::query_as(
                "SELECT id, email, name, phone, ministry_role, church_name, verified, payment_complete, selected_course_id, created_at FROM users \
                 WHERE email LIKE $1 OR name LIKE $1 OR church_name LIKE $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
            .bind(&like)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT id, email, name, phone, ministry_role, church_name, verified, payment_complete, selected_course_id, created_at FROM users \
                 WHERE email LIKE ? OR name LIKE ? OR church_name LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
            .bind(&like)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&pool)
            .await
        }
    } else if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, email, name, phone, ministry_role, church_name, verified, payment_complete, selected_course_id, created_at FROM users \
             ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, email, name, phone, ministry_role, church_name, verified, payment_complete, selected_course_id, created_at FROM users \
             ORDER BY created_at DESC LIMIT ? OFFSET ?",
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin list users error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load users"})),
        )
    })?;

    let total = count.max(0) as u64;
    Ok(Json(serde_json::json!({
        "users": users.iter().map(|u| serde_json::json!({
            "id": u.id,
            "email": u.email,
            "name": u.name,
            "phone": u.phone,
            "ministryRole": u.ministry_role,
            "churchName": u.church_name,
            "verified": u.verified,
            "paymentComplete": u.payment_complete,
            "selectedCourseId": u.selected_course_id,
            "createdAt": u.created_at,
        })).collect::<Vec<_>>(),
        "total": total
    })))
}

// ---------- Courses ----------
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CourseRow {
    pub id: String,
    pub name: String,
    pub duration: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}

async fn list_courses_admin(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<CourseRow> =
        sqlx::query_as("SELECT id, name, duration, description, price, active FROM courses")
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Admin list courses error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"message": "Failed to load courses"})),
                )
            })?;
    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "name": r.name,
                "duration": r.duration,
                "description": r.description,
                "price": r.price.unwrap_or(0.0),
                "active": r.active.unwrap_or(true),
            })
        })
        .collect();
    Ok(Json(list))
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseBody {
    pub name: String,
    pub duration: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}

async fn create_course(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    axum::Json(body): axum::Json<CreateCourseBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let name = body.name.trim();
    let duration = body.duration.as_deref().unwrap_or("").to_string();
    let description = body.description.as_deref().unwrap_or("");
    let price = body.price.unwrap_or(0.0);
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Name is required"})),
        ));
    }
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO courses (id, name, duration, description, price, active) VALUES ($1, $2, $3, $4, $5, 1)",
        )
        .bind(&id)
        .bind(name)
        .bind(&duration)
        .bind(description)
        .bind(price)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO courses (id, name, duration, description, price, active) VALUES (?, ?, ?, ?, ?, 1)",
        )
        .bind(&id)
        .bind(name)
        .bind(&duration)
        .bind(description)
        .bind(price)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin create course error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to create course"})),
        )
    })?;
    Ok(Json(serde_json::json!({
        "id": id,
        "name": name,
        "duration": duration,
        "description": description,
        "price": price,
        "active": true,
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseBody {
    pub name: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
}

async fn update_course(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateCourseBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let (name, duration, description, price, active): (String, String, String, f64, i32) =
        if db::is_postgres() {
            sqlx::query_as(
                "SELECT name, duration, COALESCE(description,''), COALESCE(price,0), COALESCE(active,1) FROM courses WHERE id = $1",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT name, duration, COALESCE(description,''), COALESCE(price,0), COALESCE(active,1) FROM courses WHERE id = ?",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
        }
        .map_err(|e| {
            tracing::error!("Admin update course fetch error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to update course"})),
            )
        })?
        .ok_or((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Course not found"})),
        ))?;
    let name = body.name.as_deref().unwrap_or(&name).to_string();
    let duration = body.duration.as_deref().unwrap_or(&duration).to_string();
    let description = body
        .description
        .as_deref()
        .unwrap_or(description.as_str())
        .to_string();
    let price = body.price.unwrap_or(price);
    let active = body.active.unwrap_or(active != 0) as i32;
    if db::is_postgres() {
        sqlx::query(
            "UPDATE courses SET name = $1, duration = $2, description = $3, price = $4, active = $5 WHERE id = $6",
        )
        .bind(&name)
        .bind(&duration)
        .bind(&description)
        .bind(price)
        .bind(active)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE courses SET name = ?, duration = ?, description = ?, price = ?, active = ? WHERE id = ?",
        )
        .bind(&name)
        .bind(&duration)
        .bind(&description)
        .bind(price)
        .bind(active)
        .bind(&id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin update course error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update course"})),
        )
    })?;
    Ok(Json(serde_json::json!({
        "id": id,
        "name": name,
        "duration": duration,
        "description": description,
        "price": price,
        "active": active != 0,
    })))
}

// ---------- Learning plans ----------
#[derive(Debug, Serialize, sqlx::FromRow)]
struct LearningPlanRow {
    id: String,
    name: String,
    description: String,
    price: f64,
    currency: String,
    duration: String,
    course_ids: String,
    active: i32,
    created_at: String,
}

async fn list_learning_plans(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<LearningPlanRow> = sqlx::query_as(
        "SELECT id, name, description, price, currency, duration, course_ids, active, created_at FROM learning_plans",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin list learning plans error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load learning plans"})),
        )
    })?;
    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            let course_ids: Vec<String> = serde_json::from_str(&r.course_ids).unwrap_or_default();
            serde_json::json!({
                "id": r.id,
                "name": r.name,
                "description": r.description,
                "price": r.price,
                "currency": r.currency,
                "duration": r.duration,
                "course_ids": course_ids,
                "active": r.active != 0,
                "created_at": r.created_at,
            })
        })
        .collect();
    Ok(Json(list))
}

#[derive(Debug, Deserialize)]
pub struct CreateLearningPlanBody {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub duration: String,
    pub course_ids: Vec<String>,
}

async fn create_learning_plan(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    axum::Json(body): axum::Json<CreateLearningPlanBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let course_ids_json =
        serde_json::to_string(&body.course_ids).unwrap_or_else(|_| "[]".to_string());
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO learning_plans (id, name, description, price, currency, duration, course_ids, active) VALUES ($1, $2, $3, $4, $5, $6, $7, 1)",
        )
        .bind(&id)
        .bind(&body.name)
        .bind(&body.description)
        .bind(body.price)
        .bind(&body.currency)
        .bind(&body.duration)
        .bind(&course_ids_json)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO learning_plans (id, name, description, price, currency, duration, course_ids, active) VALUES (?, ?, ?, ?, ?, ?, ?, 1)",
        )
        .bind(&id)
        .bind(&body.name)
        .bind(&body.description)
        .bind(body.price)
        .bind(&body.currency)
        .bind(&body.duration)
        .bind(&course_ids_json)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin create learning plan error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to create learning plan"})),
        )
    })?;
    Ok(Json(serde_json::json!({
        "id": id,
        "name": body.name,
        "description": body.description,
        "price": body.price,
        "currency": body.currency,
        "duration": body.duration,
        "course_ids": body.course_ids,
        "active": true,
        "created_at": chrono::Utc::now().to_rfc3339(),
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateLearningPlanBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub duration: Option<String>,
    pub course_ids: Option<Vec<String>>,
    pub active: Option<bool>,
}

async fn update_learning_plan(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateLearningPlanBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let row: Option<LearningPlanRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, name, description, price, currency, duration, course_ids, active, created_at FROM learning_plans WHERE id = $1",
        )
        .bind(&id)
        .fetch_optional(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, name, description, price, currency, duration, course_ids, active, created_at FROM learning_plans WHERE id = ?",
        )
        .bind(&id)
        .fetch_optional(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin update learning plan fetch error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update learning plan"})),
        )
    })?;
    let row = row.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"message": "Learning plan not found"})),
    ))?;
    let name = body.name.as_deref().unwrap_or(&row.name).to_string();
    let description = body
        .description
        .as_deref()
        .unwrap_or(&row.description)
        .to_string();
    let price = body.price.unwrap_or(row.price);
    let duration = body
        .duration
        .as_deref()
        .unwrap_or(&row.duration)
        .to_string();
    let course_ids = body
        .course_ids
        .as_ref()
        .map(|c| serde_json::to_string(c).unwrap_or_else(|_| "[]".to_string()))
        .unwrap_or(row.course_ids);
    let active = body.active.unwrap_or(row.active != 0) as i32;
    if db::is_postgres() {
        sqlx::query(
            "UPDATE learning_plans SET name = $1, description = $2, price = $3, duration = $4, course_ids = $5, active = $6 WHERE id = $7",
        )
        .bind(&name)
        .bind(&description)
        .bind(price)
        .bind(&duration)
        .bind(&course_ids)
        .bind(active)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE learning_plans SET name = ?, description = ?, price = ?, duration = ?, course_ids = ?, active = ? WHERE id = ?",
        )
        .bind(&name)
        .bind(&description)
        .bind(price)
        .bind(&duration)
        .bind(&course_ids)
        .bind(active)
        .bind(&id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin update learning plan error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update learning plan"})),
        )
    })?;
    let course_ids_vec: Vec<String> = serde_json::from_str(&course_ids).unwrap_or_default();
    Ok(Json(serde_json::json!({
        "id": id,
        "name": name,
        "description": description,
        "price": price,
        "currency": row.currency,
        "duration": duration,
        "course_ids": course_ids_vec,
        "active": active != 0,
        "created_at": row.created_at,
    })))
}

// ---------- Invites (super_admin only) ----------
#[derive(Debug, Deserialize)]
pub struct CreateInviteBody {
    pub role: String, // "admin" | "super_admin"
}

#[derive(Debug, Serialize)]
pub struct CreateInviteResponse {
    pub token: String,
    pub link: String,
    pub expires_at: String,
    pub role: String,
}

async fn create_invite(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, role): AdminUser,
    axum::Json(body): axum::Json<CreateInviteBody>,
) -> Result<Json<CreateInviteResponse>, (StatusCode, Json<serde_json::Value>)> {
    if role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"message": "Only super_admin can create invites"})),
        ));
    }
    let invite_role = body.role.trim();
    if invite_role != "admin" && invite_role != "super_admin" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "role must be admin or super_admin"})),
        ));
    }
    let id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let expires = chrono::Utc::now() + chrono::Duration::days(7);
    let expires_at = expires.to_rfc3339();
    let now = chrono::Utc::now().to_rfc3339();
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO invites (id, token, role, expires_at, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&token)
        .bind(invite_role)
        .bind(&expires_at)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO invites (id, token, role, expires_at, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&token)
        .bind(invite_role)
        .bind(&expires_at)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin create invite error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to create invite"})),
        )
    })?;
    let base =
        std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let link = format!("{}/invite/{}", base.trim_end_matches('/'), token);
    Ok(Json(CreateInviteResponse {
        token,
        link,
        expires_at,
        role: invite_role.to_string(),
    }))
}

pub fn admin_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/admin/metrics", get(metrics))
        .route("/admin/users", get(list_users))
        .route(
            "/admin/courses",
            get(list_courses_admin).post(create_course),
        )
        .route("/admin/courses/:id", patch(update_course))
        .route(
            "/admin/learning-plans",
            get(list_learning_plans).post(create_learning_plan),
        )
        .route("/admin/learning-plans/:id", patch(update_learning_plan))
        .route("/admin/invites", post(create_invite))
        .with_state(pool)
}

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
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

// ---------- Reports / Export ----------
#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

fn escape_csv_field(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

async fn export_registrations(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Query(q): Query<ExportQuery>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let want_csv = q
        .format
        .as_deref()
        .map(|f| f.eq_ignore_ascii_case("csv"))
        .unwrap_or(false);

    let users: Vec<AdminUserRow> = sqlx::query_as(
        "SELECT id, email, name, phone, ministry_role, church_name, verified, payment_complete, selected_course_id, created_at FROM users ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin export registrations error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to export registrations"})),
        )
    })?;

    if want_csv {
        let mut csv = String::from("id,email,name,phone,ministry_role,church_name,verified,payment_complete,selected_course_id,created_at\n");
        for u in &users {
            csv.push_str(&escape_csv_field(&u.id));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.email));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.name));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.phone));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.ministry_role));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.church_name));
            csv.push(',');
            csv.push_str(if u.verified { "true" } else { "false" });
            csv.push(',');
            csv.push_str(if u.payment_complete { "true" } else { "false" });
            csv.push(',');
            csv.push_str(&escape_csv_field(
                u.selected_course_id.as_deref().unwrap_or(""),
            ));
            csv.push(',');
            csv.push_str(&escape_csv_field(&u.created_at));
            csv.push('\n');
        }
        Ok(([(header::CONTENT_TYPE, "text/csv; charset=utf-8")], csv).into_response())
    } else {
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
        }))
        .into_response())
    }
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
    pub teams_class_id: Option<String>,
}

async fn list_courses_admin(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<CourseRow> = sqlx::query_as(
        "SELECT id, name, duration, description, price, active, teams_class_id FROM courses",
    )
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
                "teams_class_id": r.teams_class_id,
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
    /// Microsoft Teams class/team id for "Open in Teams" on student dashboard.
    pub teams_class_id: Option<String>,
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
    let teams_class_id = body.teams_class_id.as_deref();
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Name is required"})),
        ));
    }
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO courses (id, name, duration, description, price, active, teams_class_id) VALUES ($1, $2, $3, $4, $5, 1, $6)",
        )
        .bind(&id)
        .bind(name)
        .bind(&duration)
        .bind(description)
        .bind(price)
        .bind(teams_class_id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO courses (id, name, duration, description, price, active, teams_class_id) VALUES (?, ?, ?, ?, ?, 1, ?)",
        )
        .bind(&id)
        .bind(name)
        .bind(&duration)
        .bind(description)
        .bind(price)
        .bind(teams_class_id)
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
        "teams_class_id": body.teams_class_id,
    })))
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseBody {
    pub name: Option<String>,
    pub duration: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub active: Option<bool>,
    /// Microsoft Teams class/team id for "Open in Teams" on student dashboard.
    pub teams_class_id: Option<String>,
}

async fn update_course(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateCourseBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let (name, duration, description, price, active, existing_teams_class_id): (String, String, String, f64, i32, Option<String>) =
        if db::is_postgres() {
            sqlx::query_as(
                "SELECT name, duration, COALESCE(description,''), COALESCE(price,0), COALESCE(active,1), teams_class_id FROM courses WHERE id = $1",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT name, duration, COALESCE(description,''), COALESCE(price,0), COALESCE(active,1), teams_class_id FROM courses WHERE id = ?",
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
    let teams_class_id: Option<&str> = match body.teams_class_id.as_deref() {
        None => existing_teams_class_id.as_deref(),
        Some("") => None,
        Some(s) => Some(s),
    };
    if db::is_postgres() {
        sqlx::query(
            "UPDATE courses SET name = $1, duration = $2, description = $3, price = $4, active = $5, teams_class_id = $6 WHERE id = $7",
        )
        .bind(&name)
        .bind(&duration)
        .bind(&description)
        .bind(price)
        .bind(active)
        .bind(teams_class_id)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE courses SET name = ?, duration = ?, description = ?, price = ?, active = ?, teams_class_id = ? WHERE id = ?",
        )
        .bind(&name)
        .bind(&duration)
        .bind(&description)
        .bind(price)
        .bind(active)
        .bind(teams_class_id)
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
        "teams_class_id": teams_class_id,
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
    let rows_res: Result<Vec<LearningPlanRow>, sqlx::Error> = sqlx::query_as(
        "SELECT id, name, description, price, currency, duration, course_ids, active, created_at FROM learning_plans",
    )
    .fetch_all(&pool)
    .await;

    let rows = match rows_res {
        Ok(rows) => rows,
        Err(e) => {
            // If the table doesn't exist yet (older DB without migration 003), treat as no plans
            if let sqlx::Error::Database(db) = &e {
                let msg = db.message().to_lowercase();
                if msg.contains("no such table: learning_plans")
                    || msg.contains("relation \"learning_plans\" does not exist")
                {
                    tracing::warn!("learning_plans table missing; returning empty list");
                    return Ok(Json(Vec::new()));
                }
            }
            tracing::error!("Admin list learning plans error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to load learning plans"})),
            ));
        }
    };
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
                "courseIds": course_ids,
                "active": r.active != 0,
                "createdAt": r.created_at,
            })
        })
        .collect();
    Ok(Json(list))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        "courseIds": body.course_ids,
        "active": true,
        "createdAt": chrono::Utc::now().to_rfc3339(),
    })))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        "courseIds": course_ids_vec,
        "active": active != 0,
        "createdAt": row.created_at,
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

// ---------- Payment proofs (POP) ----------

#[derive(Debug, Serialize, sqlx::FromRow)]
struct AdminPaymentProofRow {
    id: String,
    user_id: String,
    file_url: String,
    status: String,
    note: Option<String>,
    uploaded_at: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentProofQuery {
    pub status: Option<String>, // pending | approved | rejected
    pub limit: Option<u32>,
}

async fn list_payment_proofs(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Query(q): Query<PaymentProofQuery>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let status = q.status.unwrap_or_else(|| "pending".to_string());
    let limit = q.limit.unwrap_or(20).clamp(1, 100);

    let rows: Vec<AdminPaymentProofRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, user_id, file_url, status, note, uploaded_at \
             FROM payment_proofs WHERE status = $1 ORDER BY uploaded_at DESC LIMIT $2",
        )
        .bind(&status)
        .bind(limit as i64)
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, user_id, file_url, status, note, uploaded_at \
             FROM payment_proofs WHERE status = ? ORDER BY uploaded_at DESC LIMIT ?",
        )
        .bind(&status)
        .bind(limit as i64)
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin list payment proofs error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load payment proofs"})),
        )
    })?;

    if rows.is_empty() {
        return Ok(Json(Vec::new()));
    }

    // Join with users for name/email/phone
    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let user_sql = if db::is_postgres() {
            "SELECT name, email, phone FROM users WHERE id = $1"
        } else {
            "SELECT name, email, phone FROM users WHERE id = ?"
        };
        let user_row: Option<(String, String, String)> = sqlx::query_as(user_sql)
            .bind(&row.user_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);
        let (name, email, phone) =
            user_row.unwrap_or_else(|| ("Unknown".to_string(), "".to_string(), "".to_string()));

        result.push(serde_json::json!({
            "id": row.id,
            "userId": row.user_id,
            "userName": name,
            "userEmail": email,
            "userPhone": if phone.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(phone) },
            "uploadedAt": row.uploaded_at,
            "status": row.status,
            "fileUrl": row.file_url,
            "note": row.note,
        }));
    }

    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct ReviewPaymentProofBody {
    pub status: String, // approved | rejected
    pub comment: Option<String>,
}

async fn review_payment_proof(
    State(pool): State<AnyPool>,
    AdminUser(user_id, role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<ReviewPaymentProofBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if role != "admin" && role != "super_admin" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"message": "Only admin can review payment proofs"})),
        ));
    }
    let new_status = body.status.trim();
    if new_status != "approved" && new_status != "rejected" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "status must be approved or rejected"})),
        ));
    }

    let now = chrono::Utc::now().to_rfc3339();
    let sql = if db::is_postgres() {
        "UPDATE payment_proofs SET status = $1, reviewed_at = $2, reviewed_by = $3, comment = COALESCE($4, comment) WHERE id = $5"
    } else {
        "UPDATE payment_proofs SET status = ?, reviewed_at = ?, reviewed_by = ?, comment = COALESCE(?, comment) WHERE id = ?"
    };
    sqlx::query(sql)
        .bind(new_status)
        .bind(&now)
        .bind(&user_id)
        .bind(body.comment.as_deref().unwrap_or(""))
        .bind(&id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Admin review payment proof error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Failed to update payment proof"})),
            )
        })?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// ---------- Course modules (syllabus) ----------
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

#[derive(Debug, Deserialize)]
pub struct CreateCourseModuleBody {
    pub title: String,
    pub sort_order: Option<i32>,
    pub link: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseModuleBody {
    pub title: Option<String>,
    pub sort_order: Option<i32>,
    pub link: Option<String>,
    pub description: Option<String>,
}

async fn list_course_modules_admin(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
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
        tracing::error!("Admin list course modules error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to load modules"})),
        )
    })?;
    Ok(Json(modules))
}

async fn create_course_module(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path(course_id): Path<String>,
    axum::Json(body): axum::Json<CreateCourseModuleBody>,
) -> Result<Json<CourseModuleRow>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let title = body.title.trim();
    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Title is required"})),
        ));
    }
    let sort_order = body.sort_order.unwrap_or(0);
    let link = body.link.as_deref().filter(|s| !s.is_empty());
    let description = body.description.as_deref().filter(|s| !s.is_empty());
    let now = chrono::Utc::now().to_rfc3339();
    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO course_modules (id, course_id, title, sort_order, link, description, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&id)
        .bind(&course_id)
        .bind(title)
        .bind(sort_order)
        .bind(link)
        .bind(description)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO course_modules (id, course_id, title, sort_order, link, description, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&course_id)
        .bind(title)
        .bind(sort_order)
        .bind(link)
        .bind(description)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin create course module error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to create module"})),
        )
    })?;
    Ok(Json(CourseModuleRow {
        id: id.clone(),
        course_id: course_id.clone(),
        title: title.to_string(),
        sort_order,
        link: link.map(String::from),
        description: description.map(String::from),
        created_at: now,
    }))
}

async fn update_course_module(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path((course_id, module_id)): Path<(String, String)>,
    axum::Json(body): axum::Json<UpdateCourseModuleBody>,
) -> Result<Json<CourseModuleRow>, (StatusCode, Json<serde_json::Value>)> {
    let title = body
        .title
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let sort_order = body.sort_order;
    let link = body.link.as_deref().filter(|s| !s.is_empty());
    let description = body.description.as_deref().filter(|s| !s.is_empty());
    type ModuleRow = (String, String, String, i32, Option<String>, Option<String>, String);
    let row: Option<ModuleRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, course_id, title, sort_order, link, description, created_at FROM course_modules WHERE id = $1 AND course_id = $2",
        )
        .bind(&module_id)
        .bind(&course_id)
        .fetch_optional(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, course_id, title, sort_order, link, description, created_at FROM course_modules WHERE id = ? AND course_id = ?",
        )
        .bind(&module_id)
        .bind(&course_id)
        .fetch_optional(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin update course module fetch error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update module"})),
        )
    })?;
    let mut row = row.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"message": "Module not found"})),
    ))?;
    if let Some(t) = title {
        row.2 = t.to_string();
    }
    if let Some(so) = sort_order {
        row.3 = so;
    }
    if body.link.is_some() {
        row.4 = link.map(String::from);
    }
    if body.description.is_some() {
        row.5 = description.map(String::from);
    }
    if db::is_postgres() {
        sqlx::query(
            "UPDATE course_modules SET title = $1, sort_order = $2, link = $3, description = $4 WHERE id = $5 AND course_id = $6",
        )
        .bind(&row.2)
        .bind(row.3)
        .bind(&row.4)
        .bind(&row.5)
        .bind(&module_id)
        .bind(&course_id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE course_modules SET title = ?, sort_order = ?, link = ?, description = ? WHERE id = ? AND course_id = ?",
        )
        .bind(&row.2)
        .bind(row.3)
        .bind(&row.4)
        .bind(&row.5)
        .bind(&module_id)
        .bind(&course_id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Admin update course module error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to update module"})),
        )
    })?;
    Ok(Json(CourseModuleRow {
        id: row.0,
        course_id: row.1,
        title: row.2,
        sort_order: row.3,
        link: row.4,
        description: row.5,
        created_at: row.6,
    }))
}

async fn delete_course_module(
    State(pool): State<AnyPool>,
    AdminUser(_user_id, _role): AdminUser,
    Path((course_id, module_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM course_modules WHERE id = $1 AND course_id = $2")
            .bind(&module_id)
            .bind(&course_id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM course_modules WHERE id = ? AND course_id = ?")
            .bind(&module_id)
            .bind(&course_id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Admin delete course module error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Failed to delete module"})),
        )
    })?;
    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Module not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn admin_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/admin/metrics", get(metrics))
        .route("/admin/users", get(list_users))
        .route("/admin/reports/registrations", get(export_registrations))
        .route(
            "/admin/courses",
            get(list_courses_admin).post(create_course),
        )
        .route("/admin/courses/:id", patch(update_course))
        .route(
            "/admin/courses/:course_id/modules",
            get(list_course_modules_admin).post(create_course_module),
        )
        .route(
            "/admin/courses/:course_id/modules/:module_id",
            patch(update_course_module).delete(delete_course_module),
        )
        .route(
            "/admin/learning-plans",
            get(list_learning_plans).post(create_learning_plan),
        )
        .route("/admin/learning-plans/:id", patch(update_learning_plan))
        .route("/admin/invites", post(create_invite))
        .route("/admin/payment-proofs", get(list_payment_proofs))
        .route(
            "/admin/payment-proofs/:id/review",
            post(review_payment_proof),
        )
        .with_state(pool)
}

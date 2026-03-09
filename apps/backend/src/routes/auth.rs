use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use uuid::Uuid;

use crate::auth::{create_token, AuthUser};
use crate::db;
use crate::models::{User, UserResponse};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use wangamail_rs::{BodyType, GraphMailClient, Message, MessageBody, Recipient, SendMailRequest};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub ministry_role: String,
    pub church_name: String,
    pub calling: String,
    pub selected_course_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct SelectCourseRequest {
    pub selected_course_id: Option<String>,
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| e.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed = PasswordHash::new(hash).map_err(|e| e.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

async fn send_registration_email(to: &str, name: &str) -> Result<(), String> {
    let tenant_id =
        std::env::var("AZURE_TENANT_ID").map_err(|_| "AZURE_TENANT_ID is not set".to_string())?;
    let client_id =
        std::env::var("AZURE_CLIENT_ID").map_err(|_| "AZURE_CLIENT_ID is not set".to_string())?;
    let client_secret = std::env::var("AZURE_CLIENT_SECRET")
        .map_err(|_| "AZURE_CLIENT_SECRET is not set".to_string())?;
    let from_user = std::env::var("WANGAMAIL_FROM_USER")
        .map_err(|_| "WANGAMAIL_FROM_USER is not set".to_string())?;

    let client = GraphMailClient::builder()
        .tenant_id(tenant_id)
        .client_id(client_id)
        .client_secret(client_secret)
        .build()
        .map_err(|e| e.to_string())?;

    let subject = "Welcome to Prophetic Ministry Manifestation Pastoral School";
    let body_text = format!(
        "Hi {name},\n\n\
Thank you for registering for the Prophetic Ministry Manifestation Pastoral School.\n\n\
Your account has been created successfully.\n\
Please log in with your email and password to complete your profile and verification.\n\n\
If you did not create this account, please ignore this email.\n\n\
Blessings,\n\
Prophetic Ministry Manifestation Pastoral School"
    );

    let message = Message {
        subject: subject.to_string(),
        body: MessageBody {
            content_type: BodyType::Text,
            content: body_text,
        },
        to_recipients: vec![Recipient::new(to)],
        ..Default::default()
    };

    let request = SendMailRequest::new(message);

    client
        .send_mail(&from_user, request)
        .await
        .map_err(|e| e.to_string())
}

pub fn auth_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me).patch(update_me))
        .route("/auth/verify", post(verify))
        .route("/auth/complete-payment", post(complete_payment))
        .route("/auth/courses", get(list_courses))
        .with_state(pool)
}

async fn register(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    if req.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "name_required", "message": "Name is required"})),
        ));
    }
    if req.email.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "email_required", "message": "Email is required"})),
        ));
    }
    if req.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": "password_too_short", "message": "Password must be at least 8 characters"}),
            ),
        ));
    }

    let password_hash = hash_password(&req.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "hash_failed", "message": "Could not hash password"})),
        )
    })?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let insert_sql = if db::is_postgres() {
        r#"INSERT INTO users (id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 0, 0, $10)"#
    } else {
        r#"INSERT INTO users (id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?)"#
    };

    sqlx::query(insert_sql)
        .bind(&id)
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .bind(&req.phone)
    .bind(&req.ministry_role)
    .bind(&req.church_name)
    .bind(&req.calling)
    .bind(&req.selected_course_id)
    .bind(&now)
    .execute(&pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(db) = &e {
            if db.message().contains("UNIQUE constraint failed") {
                return (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({"error": "email_taken", "message": "Email already registered"})),
                );
            }
        }
        tracing::error!("Register error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "registration_failed", "message": "Registration failed"})),
        )
    })?;

    let user = User {
        id: id.clone(),
        email: req.email,
        password_hash,
        name: req.name,
        phone: req.phone,
        ministry_role: req.ministry_role,
        church_name: req.church_name,
        calling: req.calling,
        selected_course_id: req.selected_course_id,
        verified: false,
        payment_complete: false,
        created_at: now,
    };

    // Fire-and-forget registration email via Microsoft Graph using wangamail-rs.
    // Failing to send email must not block registration.
    if let Err(e) = send_registration_email(&user.email, &user.name).await {
        tracing::error!("Registration email error for {}: {}", user.email, e);
    }

    let token = create_token(&id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "token_failed", "message": "Could not create token"})),
        )
    })?;

    Ok(Json(AuthResponse {
        user: user.to_response(),
        token,
    }))
}

async fn login(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    let login_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE email = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE email = ?"
    };
    let user: Option<User> = sqlx::query_as(login_sql)
        .bind(&req.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Login query error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "login_failed", "message": "Login failed"})),
            )
        })?;

    let user = user.ok_or((
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"error": "invalid_credentials", "message": "Invalid email or password"})),
    ))?;

    if !verify_password(&req.password, &user.password_hash).unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                serde_json::json!({"error": "invalid_credentials", "message": "Invalid email or password"}),
            ),
        ));
    }

    let token = create_token(&user.id).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "token_failed", "message": "Could not create token"})),
        )
    })?;

    Ok(Json(AuthResponse {
        user: user.to_response(),
        token,
    }))
}

async fn me(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let me_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(me_sql)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Me query error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not fetch user"}),
                ),
            )
        })?;

    let user = user.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "user_not_found", "message": "User not found"})),
    ))?;

    Ok(Json(user.to_response()))
}

async fn update_me(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    axum::Json(req): axum::Json<SelectCourseRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let update_course_sql = if db::is_postgres() {
        "UPDATE users SET selected_course_id = $1 WHERE id = $2"
    } else {
        "UPDATE users SET selected_course_id = ? WHERE id = ?"
    };
    sqlx::query(update_course_sql)
        .bind(&req.selected_course_id)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Update me error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;

    let me_sql2 = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(me_sql2)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not fetch user"}),
                ),
            )
        })?;

    Ok(Json(user.unwrap().to_response()))
}

async fn verify(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let verify_sql = if db::is_postgres() {
        "UPDATE users SET verified = 1 WHERE id = $1"
    } else {
        "UPDATE users SET verified = 1 WHERE id = ?"
    };
    sqlx::query(verify_sql)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Verify error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "verify_failed", "message": "Could not verify"})),
            )
        })?;

    let verify_select_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(verify_select_sql)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not fetch user"}),
                ),
            )
        })?;

    Ok(Json(user.unwrap().to_response()))
}

async fn complete_payment(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let payment_sql = if db::is_postgres() {
        "UPDATE users SET payment_complete = 1 WHERE id = $1"
    } else {
        "UPDATE users SET payment_complete = 1 WHERE id = ?"
    };
    sqlx::query(payment_sql)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Complete payment error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "payment_failed", "message": "Could not complete payment"})),
            )
        })?;

    let payment_select_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, created_at FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(payment_select_sql)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not fetch user"}),
                ),
            )
        })?;

    Ok(Json(user.unwrap().to_response()))
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct CourseRow {
    id: String,
    name: String,
    duration: String,
}

async fn list_courses(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<CourseRow>>, (StatusCode, Json<serde_json::Value>)> {
    let courses: Vec<CourseRow> = sqlx::query_as("SELECT id, name, duration FROM courses")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("List courses error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "fetch_failed", "message": "Could not fetch courses"})),
            )
        })?;
    Ok(Json(courses))
}

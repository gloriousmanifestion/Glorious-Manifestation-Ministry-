use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::AnyPool;
use std::sync::Arc;
use std::time::Duration;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use uuid::Uuid;

use crate::auth::{create_token, create_token_2fa_pending, verify_token, AuthUser};
use crate::db;
use crate::models::{User, UserResponse};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use totp_rs::{Algorithm, Secret, TOTP};
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
    /// When set (e.g. from /invite/[token]), validates invite and sets user role to invite role.
    pub invite_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Verify2FaRequest {
    pub temp_token: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmVerificationRequest {
    pub token: String,
}

/// PATCH /auth/me body: any subset of profile fields (snake_case from frontend).
#[derive(Debug, Default, Deserialize)]
pub struct UpdateMeRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub ministry_role: Option<String>,
    pub church_name: Option<String>,
    pub calling: Option<String>,
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

/// Send a single email via Microsoft Graph (used by registration, password reset, and admin email-students).
pub async fn send_email_to(to: &str, subject: &str, body: &str) -> Result<(), String> {
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

    let message = Message {
        subject: subject.to_string(),
        body: MessageBody {
            content_type: BodyType::Text,
            content: body.to_string(),
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

async fn send_registration_email(to: &str, name: &str) -> Result<(), String> {
    let subject = "Welcome to Prophetic Ministry Manifestation Pastoral School";
    let body = format!(
        "Hi {name},\n\n\
Thank you for registering for the Prophetic Ministry Manifestation Pastoral School.\n\n\
Your account has been created successfully.\n\
You can now log in with your email and password. After logging in, use the dashboard to request a verification email.\n\n\
If you did not create this account, please ignore this email.\n\n\
Blessings,\n\
Prophetic Ministry Manifestation Pastoral School"
    );
    send_email_to(to, subject, &body).await
}

#[derive(Debug, Serialize)]
pub struct InviteInfoResponse {
    pub valid: bool,
    pub role: Option<String>,
    pub email: Option<String>,
    pub expires_at: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
struct InviteRow {
    #[allow(dead_code)]
    id: String,
    role: String,
    email: Option<String>,
    expires_at: String,
}

pub fn auth_routes(pool: AnyPool) -> Router {
    // Stricter rate limit for login/register only: 5/min, burst 3
    let auth_strict_config = Arc::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_secs(12))
            .burst_size(3)
            .finish()
            .expect("auth governor config"),
    );
    let auth_strict = Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .layer(GovernorLayer {
            config: auth_strict_config,
        })
        .with_state(pool.clone());
    let auth_rest = Router::new()
        .route("/auth/forgot-password", post(forgot_password))
        .route("/auth/reset-password", post(reset_password))
        .route("/auth/me", get(me).patch(update_me))
        .route("/auth/verify", post(request_verification_email))
        .route("/auth/verify/confirm", post(confirm_verification))
        .route("/auth/2fa/verify", post(verify_2fa))
        .route("/auth/complete-payment", post(complete_payment))
        .route("/auth/courses", get(list_courses))
        .route("/auth/invite/:token", get(get_invite))
        .route("/payments/proofs", post(upload_payment_proof))
        .route("/payments/proofs/me", get(list_my_payment_proofs))
        .route("/payments/success", post(mark_payment_success))
        .route("/auth/me/courses/:course_id/progress", get(get_my_progress))
        .route(
            "/auth/me/courses/:course_id/modules/:module_id/complete",
            post(mark_module_complete).delete(unmark_module_complete),
        )
        .route("/me/notifications", get(get_my_notifications))
        .route("/me/notifications/:id/read", patch(mark_notification_read))
        .with_state(pool);
    auth_strict.merge(auth_rest)
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct NotificationRow {
    id: String,
    title: String,
    message: String,
    read_at: Option<String>,
    created_at: String,
}

async fn get_my_notifications(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<NotificationRow>>, (StatusCode, Json<serde_json::Value>)> {
    let sql = if db::is_postgres() {
        "SELECT id, title, message, read_at, created_at FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50"
    } else {
        "SELECT id, title, message, read_at, created_at FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT 50"
    };
    let rows = sqlx::query_as::<_, NotificationRow>(sql)
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Get notifications error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Could not load notifications"})),
            )
        })?;
    Ok(Json(rows))
}

async fn mark_notification_read(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let now = chrono::Utc::now().to_rfc3339();
    let result = if db::is_postgres() {
        sqlx::query("UPDATE notifications SET read_at = $1 WHERE id = $2 AND user_id = $3 AND read_at IS NULL")
            .bind(&now)
            .bind(&id)
            .bind(&user_id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("UPDATE notifications SET read_at = ? WHERE id = ? AND user_id = ? AND read_at IS NULL")
            .bind(&now)
            .bind(&id)
            .bind(&user_id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Mark notification read error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;
    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Notification not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn get_invite(
    State(pool): State<AnyPool>,
    Path(token): Path<String>,
) -> Json<InviteInfoResponse> {
    let bootstrap =
        std::env::var("BOOTSTRAP_INVITE_TOKEN").unwrap_or_else(|_| "bootstrap".to_string());
    if token == bootstrap {
        let has_super: bool =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE role = 'super_admin'")
                .fetch_one(&pool)
                .await
                .unwrap_or(1)
                > 0;
        return Json(InviteInfoResponse {
            valid: !has_super,
            role: Some("super_admin".to_string()),
            email: None,
            expires_at: None,
            message: if has_super {
                Some("Bootstrap already used".to_string())
            } else {
                None
            },
        });
    }
    let now = chrono::Utc::now().to_rfc3339();
    let row: Option<InviteRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, role, email, expires_at FROM invites WHERE token = $1 AND used_at IS NULL AND expires_at > $2",
        )
        .bind(&token)
        .bind(&now)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten()
    } else {
        sqlx::query_as(
            "SELECT id, role, email, expires_at FROM invites WHERE token = ? AND used_at IS NULL AND expires_at > ?",
        )
        .bind(&token)
        .bind(&now)
        .fetch_optional(&pool)
        .await
        .ok()
        .flatten()
    };
    match row {
        Some(r) => Json(InviteInfoResponse {
            valid: true,
            role: Some(r.role),
            email: r.email,
            expires_at: Some(r.expires_at),
            message: None,
        }),
        None => Json(InviteInfoResponse {
            valid: false,
            role: None,
            email: None,
            expires_at: None,
            message: Some("Invalid or expired invite".to_string()),
        }),
    }
}

/// Resolve role for registration: (role, invite_id to mark used if any).
async fn resolve_register_role(
    pool: &AnyPool,
    invite_token: Option<&str>,
) -> Result<(String, Option<String>), (StatusCode, Json<serde_json::Value>)> {
    let Some(tok) = invite_token else {
        return Ok(("student".to_string(), None));
    };
    let bootstrap =
        std::env::var("BOOTSTRAP_INVITE_TOKEN").unwrap_or_else(|_| "bootstrap".to_string());
    if tok == bootstrap {
        let has_super: bool =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE role = 'super_admin'")
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    tracing::error!("Bootstrap check error: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({"error": "bootstrap_check_failed", "message": "Could not check bootstrap"})),
                    )
                })?
                > 0;
        if has_super {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(
                    serde_json::json!({"error": "bootstrap_used", "message": "Bootstrap invite already used"}),
                ),
            ));
        }
        return Ok(("super_admin".to_string(), None));
    }
    let now = chrono::Utc::now().to_rfc3339();
    #[derive(Debug, sqlx::FromRow)]
    struct InviteIdRole {
        id: String,
        role: String,
    }
    let row: Option<InviteIdRole> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, role FROM invites WHERE token = $1 AND used_at IS NULL AND expires_at > $2",
        )
        .bind(tok)
        .bind(&now)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Invite lookup error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "invite_lookup_failed", "message": "Could not validate invite"})),
            )
        })?
    } else {
        sqlx::query_as(
            "SELECT id, role FROM invites WHERE token = ? AND used_at IS NULL AND expires_at > ?",
        )
        .bind(tok)
        .bind(&now)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Invite lookup error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "invite_lookup_failed", "message": "Could not validate invite"})),
            )
        })?
    };
    match row {
        Some(r) => Ok((r.role, Some(r.id))),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": "invalid_invite", "message": "Invalid or expired invite"}),
            ),
        )),
    }
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

    let (role, invite_id_to_use) =
        resolve_register_role(&pool, req.invite_token.as_deref()).await?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let insert_sql = if db::is_postgres() {
        r#"INSERT INTO users (id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 0, 0, $10, $11)"#
    } else {
        r#"INSERT INTO users (id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 0, ?, ?)"#
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
        .bind(&role)
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

    if let Some(invite_id) = invite_id_to_use {
        let update_invite_sql = if db::is_postgres() {
            "UPDATE invites SET used_at = $1, used_by = $2 WHERE id = $3"
        } else {
            "UPDATE invites SET used_at = ?, used_by = ? WHERE id = ?"
        };
        let _ = sqlx::query(update_invite_sql)
            .bind(&now)
            .bind(&id)
            .bind(&invite_id)
            .execute(&pool)
            .await;
    }

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
        verified: 0,
        payment_complete: 0,
        role: role.clone(),
        created_at: now,
        totp_secret: None,
    };

    // Fire-and-forget registration email via Microsoft Graph using wangamail-rs.
    // Failing to send email must not block registration.
    if let Err(e) = send_registration_email(&user.email, &user.name).await {
        tracing::error!("Registration email error for {}: {}", user.email, e);
    }

    let token = create_token(&id, &role).map_err(|_| {
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

/// POST /auth/verify – (re)send verification email for the authenticated user.
async fn request_verification_email(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Load user email/name and current verified status.
    let sql = if db::is_postgres() {
        "SELECT email, name, verified FROM users WHERE id = $1"
    } else {
        "SELECT email, name, verified FROM users WHERE id = ?"
    };
    let row: Option<(String, String, i64)> = sqlx::query_as(sql)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Request verification: fetch user error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "verify_failed", "message": "Could not start verification"})),
            )
        })?;

    let Some((email, name, verified)) = row else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "user_not_found", "message": "User not found"})),
        ));
    };

    // If already verified, just return success.
    if verified != 0 {
        return Ok(Json(serde_json::json!({
            "status": "ok",
            "message": "Account already verified."
        })));
    }

    let id = Uuid::new_v4().to_string();
    let token = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let expires_at = (now + chrono::Duration::hours(24)).to_rfc3339();
    let created_at = now.to_rfc3339();

    let insert_sql = if db::is_postgres() {
        "INSERT INTO email_verification_tokens (id, user_id, token, expires_at, created_at) VALUES ($1, $2, $3, $4, $5)"
    } else {
        "INSERT INTO email_verification_tokens (id, user_id, token, expires_at, created_at) VALUES (?, ?, ?, ?, ?)"
    };

    sqlx::query(insert_sql)
        .bind(&id)
        .bind(&user_id)
        .bind(&token)
        .bind(&expires_at)
        .bind(&created_at)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Request verification: insert token error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "verify_failed", "message": "Could not create verification token"})),
            )
        })?;

    let base_url =
        std::env::var("APP_BASE_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    let verify_link = format!(
        "{}/verify-email?token={}",
        base_url.trim_end_matches('/'),
        token
    );
    let subject = "Verify your Pastoral School account";
    let body = format!(
        "Hi {name},\n\n\
Please verify your Prophetic Ministry Manifestation Pastoral School account by clicking the link below:\n\
{verify_link}\n\n\
If you did not create this account, you can safely ignore this email.\n\n\
Blessings,\n\
Prophetic Ministry Manifestation Pastoral School"
    );

    if let Err(e) = send_email_to(&email, subject, &body).await {
        tracing::error!("Request verification email error for {}: {}", email, e);
        // Do not fail the request – avoid leaking email issues to user.
    }

    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Verification email sent. Please check your inbox."
    })))
}

/// POST /auth/verify/confirm – confirm email verification by token, mark verified, and log user in.
async fn confirm_verification(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<ConfirmVerificationRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    let token = req.token.trim();
    if token.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": "invalid_token", "message": "Invalid or missing token"}),
            ),
        ));
    }

    #[derive(Debug, sqlx::FromRow)]
    struct EmailVerifRow {
        id: String,
        user_id: String,
        #[allow(dead_code)]
        expires_at: String,
        #[allow(dead_code)]
        used_at: Option<String>,
    }

    let now = chrono::Utc::now().to_rfc3339();
    let select_sql = if db::is_postgres() {
        "SELECT id, user_id, expires_at, used_at FROM email_verification_tokens WHERE token = $1 AND (used_at IS NULL OR used_at = '') AND expires_at > $2"
    } else {
        "SELECT id, user_id, expires_at, used_at FROM email_verification_tokens WHERE token = ? AND (used_at IS NULL OR used_at = '') AND expires_at > ?"
    };

    let row: Option<EmailVerifRow> = sqlx::query_as(select_sql)
        .bind(token)
        .bind(&now)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Confirm verification: token lookup error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "verify_failed", "message": "Could not verify token"})),
            )
        })?;

    let row = row.ok_or((
        StatusCode::BAD_REQUEST,
        Json(
            serde_json::json!({"error": "invalid_token", "message": "Invalid or expired verification link"}),
        ),
    ))?;

    // Mark user as verified.
    let update_user_sql = if db::is_postgres() {
        "UPDATE users SET verified = 1 WHERE id = $1"
    } else {
        "UPDATE users SET verified = 1 WHERE id = ?"
    };
    sqlx::query(update_user_sql)
        .bind(&row.user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Confirm verification: update user error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "verify_failed", "message": "Could not verify account"}),
                ),
            )
        })?;

    // Mark token as used.
    let update_token_sql = if db::is_postgres() {
        "UPDATE email_verification_tokens SET used_at = $1 WHERE id = $2"
    } else {
        "UPDATE email_verification_tokens SET used_at = ? WHERE id = ?"
    };
    if let Err(e) = sqlx::query(update_token_sql)
        .bind(&now)
        .bind(&row.id)
        .execute(&pool)
        .await
    {
        tracing::error!("Confirm verification: mark token used error: {:?}", e);
    }

    // Load freshly updated user.
    let user_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(user_sql)
        .bind(&row.user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Confirm verification: fetch user error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not load user after verification"}),
                ),
            )
        })?;

    let user = user.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "user_not_found", "message": "User not found"})),
    ))?;

    let token = create_token(&user.id, &user.role).map_err(|_| {
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

// ---------- Payment success via redirect (no direct Selar webhook) ----------

#[derive(Debug, Deserialize)]
pub struct PaymentSuccessRequest {
    pub reference: Option<String>,
}

/// POST /payments/success – mark current authenticated user's payment as complete.
async fn mark_payment_success(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    axum::Json(req): axum::Json<PaymentSuccessRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let update_sql = if db::is_postgres() {
        "UPDATE users SET payment_complete = 1 WHERE id = $1"
    } else {
        "UPDATE users SET payment_complete = 1 WHERE id = ?"
    };
    sqlx::query(update_sql)
        .bind(&user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Mark payment success: update user error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "payment_failed", "message": "Could not update payment status"}),
                ),
            )
        })?;

    // Optional: create notification using reference
    let notif_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let title = "Payment received".to_string();
    let msg = if let Some(r) = req.reference.as_deref() {
        format!("We have recorded your payment (reference: {}).", r)
    } else {
        "We have recorded your payment.".to_string()
    };
    let notif_sql = if db::is_postgres() {
        "INSERT INTO notifications (id, user_id, title, message, created_at) VALUES ($1, $2, $3, $4, $5)"
    } else {
        "INSERT INTO notifications (id, user_id, title, message, created_at) VALUES (?, ?, ?, ?, ?)"
    };
    if let Err(e) = sqlx::query(notif_sql)
        .bind(&notif_id)
        .bind(&user_id)
        .bind(&title)
        .bind(&msg)
        .bind(&now)
        .execute(&pool)
        .await
    {
        tracing::error!("Mark payment success: insert notification error: {:?}", e);
    }

    // Return updated user
    let user_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(user_sql)
        .bind(&user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Mark payment success: fetch user error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not fetch user after payment"}),
                ),
            )
        })?;

    let user = user.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "user_not_found", "message": "User not found"})),
    ))?;

    Ok(Json(user.to_response()))
}

async fn login(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let login_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE email = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE email = ?"
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

    let requires_2fa =
        user.totp_secret.is_some() && (user.role == "admin" || user.role == "super_admin");

    if requires_2fa {
        let temp_token = create_token_2fa_pending(&user.id, &user.role).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "token_failed", "message": "Could not create token"})),
            )
        })?;
        return Ok(Json(serde_json::json!({
            "requires_2fa": true,
            "temp_token": temp_token,
        })));
    }

    let token = create_token(&user.id, &user.role).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "token_failed", "message": "Could not create token"})),
        )
    })?;

    Ok(Json(serde_json::json!({
        "user": user.to_response(),
        "token": token,
    })))
}

async fn verify_2fa(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<Verify2FaRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let claims = verify_token(req.temp_token.trim())
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "invalid_token", "message": "Invalid or expired code. Please log in again."})),
            )
        })?;
    if !claims.two_fa_pending {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": "invalid_token", "message": "Not a 2FA pending token"}),
            ),
        ));
    }

    let user_id = &claims.sub;
    let totp_secret: Option<String> = if db::is_postgres() {
        sqlx::query_scalar("SELECT totp_secret FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT totp_secret FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Fetch totp_secret error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "server_error", "message": "Could not verify"})),
        )
    })?;

    let secret = totp_secret.ok_or((
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": "no_2fa", "message": "2FA not enabled for this account"})),
    ))?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret).to_bytes().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "server_error", "message": "Invalid 2FA config"})),
            )
        })?,
        None,
        String::new(),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "server_error", "message": "Invalid 2FA config"})),
        )
    })?;

    let code = req.code.trim();
    if !totp.check_current(code).unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(
                serde_json::json!({"error": "invalid_code", "message": "Invalid or expired code"}),
            ),
        ));
    }

    let role: String = if db::is_postgres() {
        sqlx::query_scalar("SELECT role FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT role FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
    }
    .unwrap_or_else(|_| "student".to_string());

    let token = create_token(user_id, &role).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "token_failed", "message": "Could not create token"})),
        )
    })?;

    let user_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
    };
    let user: Option<User> = sqlx::query_as(user_sql)
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "server_error", "message": "Could not load user"}),
                ),
            )
        })?;

    let user = user.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "user_not_found", "message": "User not found"})),
    ))?;

    Ok(Json(serde_json::json!({
        "user": user.to_response(),
        "token": token,
    })))
}

async fn send_password_reset_email(to: &str, name: &str, reset_link: &str) -> Result<(), String> {
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

    let subject = "Reset your Prophetic Ministry Manifestation Pastoral School password";
    let body_text = format!(
        "Hi {name},\n\n\
We received a request to reset your password for the Prophetic Ministry Manifestation Pastoral School.\n\n\
You can reset your password using the link below:\n\
{reset_link}\n\n\
If you did not request a password reset, you can safely ignore this email.\n\n\
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

async fn forgot_password(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<ForgotPasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email = req.email.trim();
    if email.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "email_required",
                "message": "Email is required"
            })),
        ));
    }

    let user_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE email = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE email = ?"
    };

    let user: Option<User> = sqlx::query_as(user_sql)
        .bind(email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Forgot password query error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "forgot_password_failed", "message": "Could not process request"}),
                ),
            )
        })?;

    if let Some(user) = user {
        let token = Uuid::new_v4().to_string();
        let id = Uuid::new_v4().to_string();
        let expires_at = (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339();

        let insert_sql = if db::is_postgres() {
            "INSERT INTO password_reset_tokens (id, user_id, token, expires_at) VALUES ($1, $2, $3, $4)"
        } else {
            "INSERT INTO password_reset_tokens (id, user_id, token, expires_at) VALUES (?, ?, ?, ?)"
        };

        if let Err(e) = sqlx::query(insert_sql)
            .bind(&id)
            .bind(&user.id)
            .bind(&token)
            .bind(&expires_at)
            .execute(&pool)
            .await
        {
            tracing::error!("Insert password reset token error: {:?}", e);
        } else {
            let base_url = std::env::var("APP_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string());
            let reset_link = format!(
                "{}/reset-password?token={}",
                base_url.trim_end_matches('/'),
                token
            );

            if let Err(e) = send_password_reset_email(&user.email, &user.name, &reset_link).await {
                tracing::error!("Password reset email error for {}: {}", user.email, e);
            }
        }
    }

    // Always return success to avoid leaking which emails exist.
    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "If an account exists for that email, a reset link has been sent."
    })))
}

#[derive(Debug, sqlx::FromRow)]
struct PasswordResetRow {
    id: String,
    user_id: String,
    #[allow(dead_code)]
    expires_at: String,
    #[allow(dead_code)]
    used_at: Option<String>,
}

async fn reset_password(
    State(pool): State<AnyPool>,
    axum::Json(req): axum::Json<ResetPasswordRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if req.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": "password_too_short", "message": "Password must be at least 8 characters"}),
            ),
        ));
    }

    let now = chrono::Utc::now().to_rfc3339();

    let select_sql = if db::is_postgres() {
        "SELECT id, user_id, expires_at, used_at FROM password_reset_tokens WHERE token = $1 AND used_at IS NULL AND expires_at > $2"
    } else {
        "SELECT id, user_id, expires_at, used_at FROM password_reset_tokens WHERE token = ? AND used_at IS NULL AND expires_at > ?"
    };

    let row: Option<PasswordResetRow> = sqlx::query_as(select_sql)
        .bind(&req.token)
        .bind(&now)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Reset password token lookup error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "reset_password_failed", "message": "Could not process request"}),
                ),
            )
        })?;

    let row = row.ok_or((
        StatusCode::BAD_REQUEST,
        Json(
            serde_json::json!({"error": "invalid_token", "message": "Invalid or expired reset token"}),
        ),
    ))?;

    let new_hash = hash_password(&req.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "hash_failed", "message": "Could not hash password"})),
        )
    })?;

    let update_user_sql = if db::is_postgres() {
        "UPDATE users SET password_hash = $1 WHERE id = $2"
    } else {
        "UPDATE users SET password_hash = ? WHERE id = ?"
    };

    sqlx::query(update_user_sql)
        .bind(&new_hash)
        .bind(&row.user_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Reset password user update error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "reset_password_failed", "message": "Could not update password"}),
                ),
            )
        })?;

    let update_token_sql = if db::is_postgres() {
        "UPDATE password_reset_tokens SET used_at = $1 WHERE id = $2"
    } else {
        "UPDATE password_reset_tokens SET used_at = ? WHERE id = ?"
    };

    if let Err(e) = sqlx::query(update_token_sql)
        .bind(&now)
        .bind(&row.id)
        .execute(&pool)
        .await
    {
        tracing::error!("Reset password token mark used error: {:?}", e);
    }

    Ok(Json(serde_json::json!({
        "status": "ok",
        "message": "Password has been reset successfully."
    })))
}

async fn me(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    let me_sql = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
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
    axum::Json(req): axum::Json<UpdateMeRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<serde_json::Value>)> {
    if let Some(v) = &req.name {
        let sql = if db::is_postgres() {
            "UPDATE users SET name = $1 WHERE id = $2"
        } else {
            "UPDATE users SET name = ? WHERE id = ?"
        };
        sqlx::query(sql).bind(v).bind(&user_id).execute(&pool).await.map_err(|e| {
            tracing::error!("Update me name error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;
    }
    if let Some(v) = &req.phone {
        let sql = if db::is_postgres() {
            "UPDATE users SET phone = $1 WHERE id = $2"
        } else {
            "UPDATE users SET phone = ? WHERE id = ?"
        };
        sqlx::query(sql).bind(v).bind(&user_id).execute(&pool).await.map_err(|e| {
            tracing::error!("Update me phone error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;
    }
    if let Some(v) = &req.ministry_role {
        let sql = if db::is_postgres() {
            "UPDATE users SET ministry_role = $1 WHERE id = $2"
        } else {
            "UPDATE users SET ministry_role = ? WHERE id = ?"
        };
        sqlx::query(sql).bind(v).bind(&user_id).execute(&pool).await.map_err(|e| {
            tracing::error!("Update me ministry_role error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;
    }
    if let Some(v) = &req.church_name {
        let sql = if db::is_postgres() {
            "UPDATE users SET church_name = $1 WHERE id = $2"
        } else {
            "UPDATE users SET church_name = ? WHERE id = ?"
        };
        sqlx::query(sql).bind(v).bind(&user_id).execute(&pool).await.map_err(|e| {
            tracing::error!("Update me church_name error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;
    }
    if let Some(v) = &req.calling {
        let sql = if db::is_postgres() {
            "UPDATE users SET calling = $1 WHERE id = $2"
        } else {
            "UPDATE users SET calling = ? WHERE id = ?"
        };
        sqlx::query(sql).bind(v).bind(&user_id).execute(&pool).await.map_err(|e| {
            tracing::error!("Update me calling error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
            )
        })?;
    }
    if req.selected_course_id.is_some() {
        let sql = if db::is_postgres() {
            "UPDATE users SET selected_course_id = $1 WHERE id = $2"
        } else {
            "UPDATE users SET selected_course_id = ? WHERE id = ?"
        };
        sqlx::query(sql)
            .bind(&req.selected_course_id)
            .bind(&user_id)
            .execute(&pool)
            .await
            .map_err(|e| {
                tracing::error!("Update me selected_course_id error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "update_failed", "message": "Could not update"})),
                )
            })?;
    }

    let me_sql2 = if db::is_postgres() {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
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

#[allow(dead_code)]
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
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
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
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = $1"
    } else {
        "SELECT id, email, password_hash, name, phone, ministry_role, church_name, calling, selected_course_id, verified, payment_complete, role, created_at, totp_secret FROM users WHERE id = ?"
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
    #[serde(skip_serializing_if = "Option::is_none")]
    teams_class_id: Option<String>,
}

async fn list_courses(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<CourseRow>>, (StatusCode, Json<serde_json::Value>)> {
    let courses: Vec<CourseRow> = sqlx::query_as(
        "SELECT id, name, duration, teams_class_id FROM courses",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("List courses error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                serde_json::json!({"error": "fetch_failed", "message": "Could not fetch courses"}),
            ),
        )
    })?;
    Ok(Json(courses))
}

// ---------- Payment proofs (POP upload + listing for student) ----------

#[derive(Debug, Serialize, sqlx::FromRow)]
struct PaymentProofRow {
    id: String,
    user_id: String,
    file_url: String,
    status: String,
    note: Option<String>,
    uploaded_at: String,
    reviewed_at: Option<String>,
    reviewed_by: Option<String>,
    comment: Option<String>,
}

use axum::extract::Multipart;

async fn upload_payment_proof(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    use tokio::io::AsyncWriteExt;

    let mut file_path: Option<String> = None;
    let mut note: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        tracing::error!("Multipart error: {:?}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid_multipart", "message": "Invalid upload"})),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let file_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("pop-{}.bin", Uuid::new_v4()));
            let bytes = field.bytes().await.map_err(|e| {
                tracing::error!("Read upload error: {:?}", e);
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        serde_json::json!({"error": "upload_failed", "message": "Could not read uploaded file"}),
                    ),
                )
            })?;

            let upload_dir = std::env::var("PAYMENT_PROOFS_DIR")
                .unwrap_or_else(|_| "uploads/payment_proofs".into());
            tokio::fs::create_dir_all(&upload_dir).await.map_err(|e| {
                tracing::error!("Create upload dir error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        serde_json::json!({"error": "upload_failed", "message": "Could not prepare upload directory"}),
                    ),
                )
            })?;
            let path = format!("{}/{}", upload_dir.trim_end_matches('/'), file_name);
            let mut file = tokio::fs::File::create(&path).await.map_err(|e| {
                tracing::error!("Create upload file error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        serde_json::json!({"error": "upload_failed", "message": "Could not save uploaded file"}),
                    ),
                )
            })?;
            file.write_all(&bytes).await.map_err(|e| {
                tracing::error!("Write upload file error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        serde_json::json!({"error": "upload_failed", "message": "Could not save uploaded file"}),
                    ),
                )
            })?;
            file_path = Some(path);
        } else if name == "note" {
            let text = field.text().await.unwrap_or_default();
            if !text.trim().is_empty() {
                note = Some(text.trim().to_string());
            }
        }
    }

    let file_path = match file_path {
        Some(p) => p,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(
                    serde_json::json!({"error": "file_required", "message": "Please upload a file"}),
                ),
            ))
        }
    };

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let status = "pending".to_string();

    let insert_sql = if db::is_postgres() {
        "INSERT INTO payment_proofs (id, user_id, file_url, status, note, uploaded_at) VALUES ($1, $2, $3, $4, $5, $6)"
    } else {
        "INSERT INTO payment_proofs (id, user_id, file_url, status, note, uploaded_at) VALUES (?, ?, ?, ?, ?, ?)"
    };
    sqlx::query(insert_sql)
        .bind(&id)
        .bind(&user_id)
        .bind(&file_path)
        .bind(&status)
        .bind(&note)
        .bind(&now)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Insert payment proof error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "upload_failed", "message": "Could not save payment proof"}),
                ),
            )
        })?;

    Ok(Json(serde_json::json!({
        "id": id,
        "file_url": file_path,
        "status": status,
        "note": note,
        "uploaded_at": now,
        "reviewed_at": null,
        "reviewed_by": null,
        "comment": null
    })))
}

async fn list_my_payment_proofs(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let sql = if db::is_postgres() {
        "SELECT id, user_id, file_url, status, note, uploaded_at, reviewed_at, reviewed_by, comment \
         FROM payment_proofs WHERE user_id = $1 ORDER BY uploaded_at DESC"
    } else {
        "SELECT id, user_id, file_url, status, note, uploaded_at, reviewed_at, reviewed_by, comment \
         FROM payment_proofs WHERE user_id = ? ORDER BY uploaded_at DESC"
    };
    let rows: Vec<PaymentProofRow> = sqlx::query_as(sql)
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("List my payment proofs error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not load payment proofs"}),
                ),
            )
        })?;
    let list: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "id": r.id,
                "file_url": r.file_url,
                "status": r.status,
                "note": r.note,
                "uploaded_at": r.uploaded_at,
                "reviewed_at": r.reviewed_at,
                "reviewed_by": r.reviewed_by,
                "comment": r.comment,
            })
        })
        .collect();
    Ok(Json(list))
}

// ---------- Progress (module completions) ----------

#[derive(Debug, Serialize)]
struct ProgressResponse {
    completed_module_ids: Vec<String>,
}

async fn get_my_progress(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path(course_id): Path<String>,
) -> Result<Json<ProgressResponse>, (StatusCode, Json<serde_json::Value>)> {
    let sql = if db::is_postgres() {
        "SELECT mc.course_module_id FROM module_completions mc
         INNER JOIN course_modules m ON m.id = mc.course_module_id AND m.course_id = $1
         WHERE mc.user_id = $2 ORDER BY mc.completed_at"
    } else {
        "SELECT mc.course_module_id FROM module_completions mc
         INNER JOIN course_modules m ON m.id = mc.course_module_id AND m.course_id = ?
         WHERE mc.user_id = ? ORDER BY mc.completed_at"
    };
    let rows: Vec<(String,)> = sqlx::query_as(sql)
        .bind(&course_id)
        .bind(&user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Get my progress error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "fetch_failed", "message": "Could not load progress"}),
                ),
            )
        })?;
    let completed_module_ids: Vec<String> = rows.into_iter().map(|r| r.0).collect();
    Ok(Json(ProgressResponse {
        completed_module_ids,
    }))
}

async fn mark_module_complete(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path((course_id, module_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Ensure module belongs to course
    let check_sql = if db::is_postgres() {
        "SELECT id FROM course_modules WHERE id = $1 AND course_id = $2"
    } else {
        "SELECT id FROM course_modules WHERE id = ? AND course_id = ?"
    };
    let exists: Option<(String,)> = sqlx::query_as(check_sql)
        .bind(&module_id)
        .bind(&course_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Check module error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "db_error", "message": "Could not verify module"}),
                ),
            )
        })?;
    if exists.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(
                serde_json::json!({"error": "not_found", "message": "Module not found for this course"}),
            ),
        ));
    }
    let now = chrono::Utc::now().to_rfc3339();
    let insert_sql = if db::is_postgres() {
        "INSERT INTO module_completions (user_id, course_module_id, completed_at) VALUES ($1, $2, $3) ON CONFLICT (user_id, course_module_id) DO NOTHING"
    } else {
        "INSERT OR IGNORE INTO module_completions (user_id, course_module_id, completed_at) VALUES (?, ?, ?)"
    };
    sqlx::query(insert_sql)
        .bind(&user_id)
        .bind(&module_id)
        .bind(&now)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Mark complete error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "update_failed", "message": "Could not mark module complete"}),
                ),
            )
        })?;
    Ok(StatusCode::NO_CONTENT)
}

async fn unmark_module_complete(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path((course_id, module_id)): Path<(String, String)>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Optional: verify module belongs to course (for consistency)
    let delete_sql = if db::is_postgres() {
        "DELETE FROM module_completions WHERE user_id = $1 AND course_module_id = $2 AND course_module_id IN (SELECT id FROM course_modules WHERE course_id = $3)"
    } else {
        "DELETE FROM module_completions WHERE user_id = ? AND course_module_id = ? AND course_module_id IN (SELECT id FROM course_modules WHERE course_id = ?)"
    };
    let _ = sqlx::query(delete_sql)
        .bind(&user_id)
        .bind(&module_id)
        .bind(&course_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Unmark complete error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    serde_json::json!({"error": "update_failed", "message": "Could not unmark module"}),
                ),
            )
        })?;
    Ok(StatusCode::NO_CONTENT)
}

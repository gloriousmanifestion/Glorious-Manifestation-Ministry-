//! Blog/News: public list + get by slug, admin CRUD, comments.

#[allow(unused_imports)]
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
pub struct BlogPostRow {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub author_name: String,
    pub image_url: Option<String>,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

async fn list_public(
    State(pool): State<AnyPool>,
) -> Result<Json<Vec<BlogPostRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<BlogPostRow> = sqlx::query_as(
        "SELECT id, title, slug, body, author_name, image_url, published_at, created_at, updated_at \
         FROM blog_posts WHERE published_at IS NOT NULL ORDER BY published_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("List blog posts error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch posts"})),
        )
    })?;
    Ok(Json(rows))
}

async fn get_by_slug(
    State(pool): State<AnyPool>,
    Path(slug): Path<String>,
) -> Result<Json<BlogPostRow>, (StatusCode, Json<serde_json::Value>)> {
    let row: Option<BlogPostRow> = if db::is_postgres() {
        sqlx::query_as(
            "SELECT id, title, slug, body, author_name, image_url, published_at, created_at, updated_at \
             FROM blog_posts WHERE slug = $1 AND published_at IS NOT NULL",
        )
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT id, title, slug, body, author_name, image_url, published_at, created_at, updated_at \
             FROM blog_posts WHERE slug = ? AND published_at IS NOT NULL",
        )
        .bind(&slug)
        .fetch_optional(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Get blog post error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch post"})),
        )
    })?;

    let post = row.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"message": "Post not found"})),
    ))?;
    Ok(Json(post))
}

// ---------- Comments ----------
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct BlogCommentRow {
    pub id: String,
    pub blog_post_id: String,
    pub user_id: String,
    pub author_name: String,
    pub body: String,
    pub created_at: String,
}

async fn list_comments(
    State(pool): State<AnyPool>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<BlogCommentRow>>, (StatusCode, Json<serde_json::Value>)> {
    let post_id: Option<String> = if db::is_postgres() {
        sqlx::query_scalar("SELECT id FROM blog_posts WHERE slug = $1 AND published_at IS NOT NULL")
            .bind(&slug)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT id FROM blog_posts WHERE slug = ? AND published_at IS NOT NULL")
            .bind(&slug)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Resolve slug for comments: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not load comments"})),
        )
    })?;

    let post_id = match post_id {
        Some(id) => id,
        None => {
            return Ok(Json(vec![]));
        }
    };

    let sql = if db::is_postgres() {
        "SELECT c.id, c.blog_post_id, c.user_id, COALESCE(u.name, 'User') AS author_name, c.body, c.created_at \
         FROM blog_comments c LEFT JOIN users u ON u.id = c.user_id \
         WHERE c.blog_post_id = $1 ORDER BY c.created_at ASC"
    } else {
        "SELECT c.id, c.blog_post_id, c.user_id, COALESCE(u.name, 'User') AS author_name, c.body, c.created_at \
         FROM blog_comments c LEFT JOIN users u ON u.id = c.user_id \
         WHERE c.blog_post_id = ? ORDER BY c.created_at ASC"
    };
    let rows: Vec<BlogCommentRow> = sqlx::query_as(sql)
        .bind(&post_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            tracing::error!("List comments error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Could not load comments"})),
            )
        })?;
    Ok(Json(rows))
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentBody {
    pub body: String,
}

async fn add_comment(
    State(pool): State<AnyPool>,
    AuthUser(user_id): AuthUser,
    Path(slug): Path<String>,
    axum::Json(body): axum::Json<CreateCommentBody>,
) -> Result<Json<BlogCommentRow>, (StatusCode, Json<serde_json::Value>)> {
    let body_text = body.body.trim();
    if body_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Comment body is required"})),
        ));
    }

    let post_id: Option<String> = if db::is_postgres() {
        sqlx::query_scalar("SELECT id FROM blog_posts WHERE slug = $1 AND published_at IS NOT NULL")
            .bind(&slug)
            .fetch_optional(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT id FROM blog_posts WHERE slug = ? AND published_at IS NOT NULL")
            .bind(&slug)
            .fetch_optional(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Resolve slug for add comment: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not add comment"})),
        )
    })?;

    let post_id = post_id.ok_or((
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"message": "Post not found"})),
    ))?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO blog_comments (id, blog_post_id, user_id, body, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&id)
        .bind(&post_id)
        .bind(&user_id)
        .bind(body_text)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO blog_comments (id, blog_post_id, user_id, body, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&post_id)
        .bind(&user_id)
        .bind(body_text)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Insert comment error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not add comment"})),
        )
    })?;

    let author_name: String = if db::is_postgres() {
        sqlx::query_scalar("SELECT name FROM users WHERE id = $1")
            .bind(&user_id)
            .fetch_optional(&pool)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "User".to_string())
    } else {
        sqlx::query_scalar("SELECT name FROM users WHERE id = ?")
            .bind(&user_id)
            .fetch_optional(&pool)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| "User".to_string())
    };

    Ok(Json(BlogCommentRow {
        id,
        blog_post_id: post_id,
        user_id,
        author_name,
        body: body_text.to_string(),
        created_at: now,
    }))
}

async fn list_admin(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
) -> Result<Json<Vec<BlogPostRow>>, (StatusCode, Json<serde_json::Value>)> {
    let rows: Vec<BlogPostRow> = sqlx::query_as(
        "SELECT id, title, slug, body, author_name, image_url, published_at, created_at, updated_at \
         FROM blog_posts ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Admin list blog error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not fetch posts"})),
        )
    })?;
    Ok(Json(rows))
}

fn slugify(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[derive(Debug, Deserialize)]
pub struct CreateBlogPostBody {
    pub title: String,
    pub slug: Option<String>,
    pub body: String,
    pub author_name: String,
    pub image_url: Option<String>,
    pub published_at: Option<String>,
}

async fn create(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    axum::Json(body): axum::Json<CreateBlogPostBody>,
) -> Result<Json<BlogPostRow>, (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let title = body.title.trim();
    let slug = body
        .slug
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(slugify)
        .unwrap_or_else(|| slugify(title));
    let body_text = body.body.trim();
    let author_name = body.author_name.trim();
    let image_url = body
        .image_url
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from);
    let published_at = body
        .published_at
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from);
    let now = chrono::Utc::now().to_rfc3339();

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Title is required"})),
        ));
    }
    if slug.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Slug is required"})),
        ));
    }
    if body_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Body is required"})),
        ));
    }
    if author_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Author name is required"})),
        ));
    }

    if db::is_postgres() {
        sqlx::query(
            "INSERT INTO blog_posts (id, title, slug, body, author_name, image_url, published_at, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(&id)
        .bind(title)
        .bind(&slug)
        .bind(body_text)
        .bind(author_name)
        .bind(image_url.as_deref())
        .bind(published_at.as_deref())
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "INSERT INTO blog_posts (id, title, slug, body, author_name, image_url, published_at, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(title)
        .bind(&slug)
        .bind(body_text)
        .bind(author_name)
        .bind(image_url.as_deref())
        .bind(published_at.as_deref())
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Create blog post error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not create post"})),
        )
    })?;

    Ok(Json(BlogPostRow {
        id: id.clone(),
        title: title.to_string(),
        slug: slug.clone(),
        body: body_text.to_string(),
        author_name: author_name.to_string(),
        image_url: image_url.clone(),
        published_at: published_at.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateBlogPostBody {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub body: Option<String>,
    pub author_name: Option<String>,
    pub image_url: Option<String>,
    pub published_at: Option<String>,
}

async fn update(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
    axum::Json(body): axum::Json<UpdateBlogPostBody>,
) -> Result<Json<BlogPostRow>, (StatusCode, Json<serde_json::Value>)> {
    type BlogPostParts = (
        String,
        String,
        String,
        String,
        Option<String>,
        Option<String>,
    );
    let existing: Option<BlogPostParts> = if db::is_postgres() {
            sqlx::query_as(
                "SELECT title, slug, body, author_name, image_url, published_at FROM blog_posts WHERE id = $1",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT title, slug, body, author_name, image_url, published_at FROM blog_posts WHERE id = ?",
            )
            .bind(&id)
            .fetch_optional(&pool)
            .await
        }
        .map_err(|e| {
            tracing::error!("Fetch blog post error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"message": "Could not update"})),
            )
        })?;

    let (title, slug, body_text, author_name, image_url, published_at) = match existing {
        Some((t, s, b, a, img, pub_at)) => (
            body.title.as_deref().unwrap_or(&t).trim().to_string(),
            body.slug
                .as_deref()
                .map(|x| slugify(x.trim()))
                .filter(|x| !x.is_empty())
                .unwrap_or(s),
            body.body.as_deref().unwrap_or(&b).trim().to_string(),
            body.author_name.as_deref().unwrap_or(&a).trim().to_string(),
            body.image_url
                .as_deref()
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(String::from)
                .or(img),
            body.published_at
                .as_deref()
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(String::from)
                .or(pub_at),
        ),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "Post not found"})),
            ));
        }
    };

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Title is required"})),
        ));
    }
    if slug.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Slug is required"})),
        ));
    }
    if body_text.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Body is required"})),
        ));
    }
    if author_name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"message": "Author name is required"})),
        ));
    }

    let now = chrono::Utc::now().to_rfc3339();

    if db::is_postgres() {
        sqlx::query(
            "UPDATE blog_posts SET title = $1, slug = $2, body = $3, author_name = $4, image_url = $5, published_at = $6, updated_at = $7 WHERE id = $8",
        )
        .bind(&title)
        .bind(&slug)
        .bind(&body_text)
        .bind(&author_name)
        .bind(image_url.as_deref())
        .bind(published_at.as_deref())
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await
    } else {
        sqlx::query(
            "UPDATE blog_posts SET title = ?, slug = ?, body = ?, author_name = ?, image_url = ?, published_at = ?, updated_at = ? WHERE id = ?",
        )
        .bind(&title)
        .bind(&slug)
        .bind(&body_text)
        .bind(&author_name)
        .bind(image_url.as_deref())
        .bind(published_at.as_deref())
        .bind(&now)
        .bind(&id)
        .execute(&pool)
        .await
    }
    .map_err(|e| {
        tracing::error!("Update blog post error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not update"})),
        )
    })?;

    let created_at: String = if db::is_postgres() {
        sqlx::query_scalar("SELECT created_at FROM blog_posts WHERE id = $1")
            .bind(&id)
            .fetch_one(&pool)
            .await
    } else {
        sqlx::query_scalar("SELECT created_at FROM blog_posts WHERE id = ?")
            .bind(&id)
            .fetch_one(&pool)
            .await
    }
    .unwrap_or_else(|_| now.clone());

    Ok(Json(BlogPostRow {
        id,
        title,
        slug,
        body: body_text,
        author_name,
        image_url,
        published_at,
        created_at,
        updated_at: now,
    }))
}

async fn delete_one(
    State(pool): State<AnyPool>,
    AdminUser(_uid, _role): AdminUser,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let result = if db::is_postgres() {
        sqlx::query("DELETE FROM blog_posts WHERE id = $1")
            .bind(&id)
            .execute(&pool)
            .await
    } else {
        sqlx::query("DELETE FROM blog_posts WHERE id = ?")
            .bind(&id)
            .execute(&pool)
            .await
    }
    .map_err(|e| {
        tracing::error!("Delete blog post error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"message": "Could not delete"})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"message": "Post not found"})),
        ));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub fn blog_routes(pool: AnyPool) -> Router {
    Router::new()
        .route("/blog", get(list_public))
        .route("/blog/:slug", get(get_by_slug))
        .route("/blog/:slug/comments", get(list_comments).post(add_comment))
        .route("/admin/blog", get(list_admin).post(create))
        .route("/admin/blog/:id", patch(update).delete(delete_one))
        .with_state(pool)
}

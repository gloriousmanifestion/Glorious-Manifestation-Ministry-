use axum::{response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod auth;
mod db;
mod models;
mod routes;

#[derive(Serialize, Deserialize)]
struct ApiInfo {
    name: String,
    version: String,
    endpoints: Vec<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("backend=debug,tower_http=debug")
        .init();

    let pool: sqlx::AnyPool = db::init_db().await.expect("Failed to init database");

    let app = Router::new()
        .route("/", get(root))
        .route("/api/v1/info", get(api_info))
        .merge(routes::health_routes(pool.clone()))
        .nest(
            "/api/v1",
            routes::auth_routes(pool.clone())
                .merge(routes::admin_routes(pool.clone()))
                .merge(routes::events_routes(pool.clone()))
                .merge(routes::church_routes(pool.clone()))
                .merge(routes::announcements_routes(pool.clone()))
                .merge(routes::testimonials_routes(pool.clone()))
                .merge(routes::blog_routes(pool.clone()))
                .merge(routes::newsletter_routes(pool.clone()))
                .merge(routes::contact_routes(pool)),
        )
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(6830);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Pastoral School Backend",
        "description": "API for Prophetic Ministry Manifestation Pastoral School",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health": "/health",
            "api_info": "/api/v1/info",
            "auth_register": "/api/v1/auth/register",
            "auth_login": "/api/v1/auth/login",
            "auth_me": "/api/v1/auth/me",
            "auth_verify": "/api/v1/auth/verify",
            "auth_complete_payment": "/api/v1/auth/complete-payment",
            "auth_courses": "/api/v1/auth/courses",
            "events": "/api/v1/events",
            "contact": "/api/v1/contact"
        }
    }))
}

async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "Pastoral School API".to_string(),
        version: "v1".to_string(),
        endpoints: vec![
            "/api/v1/auth/register".to_string(),
            "/api/v1/auth/login".to_string(),
            "/api/v1/auth/me".to_string(),
            "/api/v1/auth/verify".to_string(),
            "/api/v1/auth/complete-payment".to_string(),
            "/api/v1/auth/courses".to_string(),
        ],
    })
}

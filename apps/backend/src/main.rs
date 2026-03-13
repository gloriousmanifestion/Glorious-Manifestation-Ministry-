use axum::{response::Json, routing::{get, post}, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use axum::http::{Method, HeaderValue};
use tower_http::trace::TraceLayer;

mod auth;
mod db;
mod models;
mod routes;

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct ApiInfo {
    name: String,
    version: String,
    endpoints: Vec<String>,
}

#[tokio::main]
async fn main() {
    // Enable tracing for debugging
    tracing_subscriber::fmt()
        .with_env_filter("backend=debug,tower_http=debug")
        .init();

    // Initialize database pool
    let pool: sqlx::AnyPool = db::init_db().await.expect("Failed to init database");

    // Configure production-safe CORS
    let cors = CorsLayer::new()
        .allow_origin("https://glorious-manifestation-ministry.vercel.app"
            .parse::<HeaderValue>()
            .unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/v1/info", get(api_info))
        .nest(
            "/api/v1",
            routes::auth_routes(pool.clone())
                .merge(routes::admin_routes(pool.clone()))
                .merge(routes::events_routes(pool.clone()))
                .merge(routes::contact_routes(pool)),
        )
        .layer(cors)                     // ✅ Attach production CORS
        .layer(TraceLayer::new_for_http());

    // Set server port
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(6830);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Root endpoint
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

// Health endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

// API info endpoint
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

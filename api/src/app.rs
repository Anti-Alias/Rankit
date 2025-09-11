use axum::Router;
use axum::routing::get;

pub fn app_router() -> Router {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "API is healthy"
}

mod middleware;

use axum::{routing::get, Router};
use tower_http::cors::{CorsLayer, Any};
use crate::handler;
pub fn init_routers() -> Router {
    Router::new()
        .fallback(handler::posts::error_page)
        .nest("/", public_routers())
        .nest("/", private_routers())
        // 允许所有来源的请求，跨域中间件
        .layer(CorsLayer::new().allow_origin(Any))
}

fn public_routers() -> Router {
    Router::new()
        .route("/", get(handler::posts::index))
        .route("/index", get(handler::posts::index))
        
}

fn private_routers() -> Router {
    Router::new()
        .route("/admin", get(|| async { "Hello, World!" }))
}
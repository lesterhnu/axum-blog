mod admin;
mod middleware;
use crate::handler;
use axum::{ routing::{get,post}, Router};
use tower_http::cors::{Any, CorsLayer};
pub fn init_routers() -> Router {
    Router::new()
        .fallback(handler::posts::error_page)
        .nest("/", public_routers())
        .nest("/", private_routers())
        // 允许所有来源的请求，跨域中间件
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(axum::middleware::from_fn(middleware::uri))
}

fn public_routers() -> Router {
    Router::new()
        .route("/", get(handler::posts::index))
        .route("/index", get(handler::posts::index))
}

fn private_routers() -> Router {
    Router::new()
        .route("/admin", get(|| async { "Hello, World!" }))
        .route("/admin/edit-post", get(handler::posts::edit_post_page))
        .route("/admin/post/save", post(handler::posts::save_post))
}

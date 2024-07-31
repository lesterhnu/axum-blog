mod admin;
mod middleware;
use crate::handler;
use axum::{
    routing::{get, post},
    Router,
};
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
        .route("/post/:id", get(handler::posts::get_post_detail))
        .route("/get_error", get(handler::posts::get_error))
        .route("/about", get(handler::index::about_page))
        .route("/admin/login", get(handler::admin::login_ui))
}

fn private_routers() -> Router {
    Router::new()
        .route("/admin", get(|| async { "Hello, World!" }))
        .route("/admin/index", get(handler::admin::index))
        .route("/admin/edit-post", get(handler::posts::edit_post_page))
        .route("/admin/post/save", post(handler::posts::save_post))
        .route("/admin/login", post(handler::admin::login))
}

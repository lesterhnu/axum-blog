use crate::{dto, Result,dao};
use askama_axum::Template;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::Form;

#[derive(Debug, Template)]
#[template(path = "backend/login.html")]
pub struct LoginTemplate;

pub async fn login_ui() -> Result<LoginTemplate> {
    Ok(LoginTemplate {})
}
pub async fn login(Form(req): Form<dto::admin::LoginReq>) -> Result<(StatusCode, HeaderMap)> {
    tracing::info!("{:#?}", req);
    super::redirect_with_cookie()
    // Redirect::to("/admin/index")
}

#[derive(Debug, Template)]
#[template(path = "backend/index.html")]
pub struct AdminIndexTemplate{
    pub posts:Vec<dto::posts::PostsWithTag>
}

pub async fn index() -> Result<AdminIndexTemplate> {
    let posts = dao::posts::get_posts().await?;
    Ok(AdminIndexTemplate {posts})
}

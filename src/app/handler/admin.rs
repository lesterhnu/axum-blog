use askama_axum::Template;
use axum::http::HeaderMap;
use axum::{http::StatusCode, response::Redirect};
use axum::Form;
use crate::{dto, Result};

#[derive(Debug,Template)]
#[template(path = "backend/login.html")]
pub struct LoginTemplate;

pub async fn login_ui() -> Result<LoginTemplate> {
    Ok(LoginTemplate {})
}
pub async fn login(Form(req):Form<dto::admin::LoginReq>)->Result<(StatusCode,HeaderMap)>{
    tracing::info!("{:#?}",req);
    super::redirect_with_cookie()
    // Redirect::to("/admin/index")
}


#[derive(Debug,Template)]
#[template(path = "backend/index.html")]
pub struct AdminIndexTemplate;


pub async fn index()->Result<AdminIndexTemplate>{
    Ok(AdminIndexTemplate{})
}
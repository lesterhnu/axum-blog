use axum::response::{Html,Response};
use thiserror::Error;
use askama_axum::Template;


#[derive(Debug,Error)]
pub enum MyError{
    #[error("error info:{1}")]
    WithCodeMsg(i32,String),
    #[error("default error")]
    Default,
    #[error("dberror:{0}")]
    DbError(#[from] sea_orm::DbErr),
}
#[derive(Debug,Template)]
#[template(path = "frontend/error.html")]
pub struct ErrorPage;


impl axum::response::IntoResponse for MyError{
    fn into_response(self) -> Response {
        Html(ErrorPage.render().unwrap()).into_response()
    }
}
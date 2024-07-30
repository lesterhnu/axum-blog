use askama_axum::Template;
use axum::response::{Html, Response};
use thiserror::Error;
use axum::Json;
use serde::Serialize;


#[derive(Debug, Error)]
pub enum MyError {
    #[error("error info:{1}")]
    WithCodeMsg(i32, String),
    #[error("default error")]
    Default,
    #[error("dberror:{0}")]
    DbError(#[from] sea_orm::DbErr),
}
impl MyError {
    pub fn with_code_msg() -> Self {
        MyError::Default
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResp{
    pub code:i32,
    pub msg:String,
}
impl axum::response::IntoResponse for ErrorResp {
    fn into_response(self) -> Response {
        todo!()
    }
}


#[derive(Debug, Template)]
#[template(path = "frontend/error.html")]
pub struct ErrorPage;

impl axum::response::IntoResponse for MyError {
    fn into_response(self) -> Response {
        match self{
            MyError::WithCodeMsg(code,msg)=>{
                let resp = ErrorResp{
                    code,
                    msg,
                };
                Json(resp).into_response()
            },
            MyError::Default =>{
                let resp = ErrorResp{
                    code:-1,
                    msg:self.to_string(),
                };
                Json(resp).into_response()
            }
            _ => Html(ErrorPage.render().unwrap()).into_response()
        }
    }
}

use crate::dao;
use crate::dto;
use crate::dto::posts::PostsWithTag;
use crate::MyError;
use crate::Result;
use askama_axum::Template;
use axum::extract::Request;
use axum::Form;

#[derive(Template, Debug)]
#[template(path = "frontend/index.html")]
pub struct Index {
    pub posts: Vec<PostsWithTag>,
}

pub async fn index() -> Result<Index> {
    let p = dao::posts::get_posts().await?;
    tracing::info!("{:?}", p);
    Ok(Index { posts: p })
}

#[derive(Template, Debug)]
#[template(path = "frontend/error.html")]
pub struct ErrorPage;

pub async fn error_page(req:Request) -> Result<ErrorPage> {
    tracing::warn!("NOT FOUND:{}",req.uri());
    Ok(ErrorPage)
}

#[derive(Debug, Template)]
#[template(path = "backend/edit_post.html")]
pub struct EditPostTemplate {}

pub async fn edit_post_page() -> Result<EditPostTemplate> {
    Ok(EditPostTemplate {})
}

#[derive(Debug, Template)]
#[template(path = "backend/save_post.html")]
pub struct SavePostTemplate {
    pub msg: String,
}
pub async fn save_post(Form(post_req): Form<dto::posts::PostReq>) -> Result<SavePostTemplate> {
    let title = post_req
        .title
        .ok_or(MyError::WithCodeMsg(-1, "title error".to_string()))?;
    let content = post_req
        .content
        .ok_or(MyError::WithCodeMsg(-1, "content error".to_string()))?;
    dao::posts::create_post(title, content).await?;
    Ok(SavePostTemplate {
        msg: "success".to_string(),
    })
}


pub async fn get_error()->MyError{
    tracing::info!("dddd");
    // Ok(())
    MyError::WithCodeMsg(-1,"error".to_string())
}
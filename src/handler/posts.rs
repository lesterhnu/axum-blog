use askama_axum::Template;
use crate::Result;

#[derive(Template,Debug)]
#[template(path = "frontend/index.html")]
pub struct Index;

pub async fn index() ->  Result<Index> {
    Ok(Index)
}

#[derive(Template,Debug)]
#[template(path = "frontend/error.html")]
pub struct ErrorPage;

pub async fn error_page() ->  Result<ErrorPage> {
    Ok(ErrorPage)
}
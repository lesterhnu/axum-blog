use crate::dto::posts::PostsWithTag;
use crate::Result;
use crate::dao;
use askama_axum::Template;

#[derive(Template, Debug)]
#[template(path = "frontend/index.html")]
pub struct Index {
    pub posts: Vec<PostsWithTag>,
}

pub async fn index() -> Result<Index> {
    let p = dao::posts::get_posts().await?;
    tracing::info!("{:?}",p);
    Ok(Index { posts: p })
}

#[derive(Template, Debug)]
#[template(path = "frontend/error.html")]
pub struct ErrorPage;

pub async fn error_page() -> Result<ErrorPage> {
    Ok(ErrorPage)
}

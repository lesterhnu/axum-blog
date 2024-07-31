use askama_axum::Template;
use crate::Result;

#[derive(Debug,Template)]
#[template(path = "frontend/about.html")]
pub struct AboutTemplate;

pub async fn about_page() -> Result<AboutTemplate> {
    Ok(AboutTemplate {})
}

#[derive(Debug,Template)]
#[template(path = "frontend/search_result.html")]
pub struct SearchResultTemplate{}

pub async fn search_page() -> Result<SearchResultTemplate> {
    Ok(SearchResultTemplate {})
}
use crate::Result;
use axum::http::{
    header::{self, SET_COOKIE},
    HeaderMap, StatusCode,
};
pub mod admin;
pub mod index;
pub mod posts;

pub fn redirect_with_cookie() -> Result<(StatusCode, HeaderMap)> {
    let mut hm = HeaderMap::new();
    hm.insert(SET_COOKIE, "axum_blog_cookie=admincookie".parse().unwrap());
    hm.insert(header::LOCATION, "/admin/index".parse().unwrap());
    Ok((StatusCode::FOUND, hm))
}

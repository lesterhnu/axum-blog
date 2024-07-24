use axum::{extract::Request, middleware::Next, response::Response};
use crate::Result;


#[allow(unused)]
pub async fn check_jwt(req:Request,next:Next)->Result<Response>{
    todo!()
}
use axum::{extract::Request, middleware::Next, response::Response};
use crate::Result;


#[allow(unused)]
pub async fn check_jwt(req:Request,next:Next)->Result<Response>{
    todo!()
}

pub async fn uri(req:Request,next:Next)->Result<Response>{
    println!("uri:{}",req.uri());
    tracing::info!("uri:{}",req.uri());
    Ok(next.run(req).await)
}
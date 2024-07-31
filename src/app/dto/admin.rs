use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}
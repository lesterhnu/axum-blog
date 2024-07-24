use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug,Serialize,Deserialize)]
pub struct PostsWithTag {
    pub post: models::post::Model,
    pub tags:Vec<models::tags::Model>,
}
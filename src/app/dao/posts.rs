
use sea_orm::EntityTrait;

use crate::models::post;
use crate::models::tags;
use crate::Result;
use crate::dto::posts::PostsWithTag;

pub async fn get_posts()->Result<Vec<PostsWithTag>>{
    let mut result = Vec::<PostsWithTag>::new();
    let db = crate::get_db()?;
    let res = post::Entity::find()
    .find_with_related(tags::Entity)
    .all(db).await?;
    for (p,t) in res.into_iter(){
        result.push(PostsWithTag{
            post:p,
            tags:t,
        })
    }
    
    Ok(result)
}
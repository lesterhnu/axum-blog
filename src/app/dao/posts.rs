
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::Set;

use crate::models::post;
use crate::models::tags;
use crate::{Result,MyError};
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
pub async fn get_post_by_id(id:i32)->Result<PostsWithTag>{
    let db = crate::get_db()?;
    let res = post::Entity::find_by_id(id)
    .find_with_related(tags::Entity)
    .all(db).await?;
    if res.is_empty(){
        return Err(MyError::WithCodeMsg(-1,"not found".to_string()))
    }

    if let Some((p,t)) = res.get(0){
        Ok(PostsWithTag{
            post:p.clone(),
            tags:t.clone(),
        })
    }else{
        Err(MyError::WithCodeMsg(-1,"not found".to_string()))
    }
}

pub async fn create_post(title:String,content:String)->Result<i32>{
    let db = crate::get_db()?;
    let am  = post::ActiveModel{
        title:Set(title),
        text:Set(content),
        ..Default::default()
    };
    let res = am.insert(db).await?;
    Ok(res.id)
}
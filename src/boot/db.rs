use tokio::sync::OnceCell;
use sea_orm::{DatabaseConnection,Database};
use std::time::Duration;

use crate::{MyError,Result};


pub static DB:OnceCell<DatabaseConnection> = OnceCell::const_new();
// const SQLITE_DSN:&'static str = "sqlite:///Users/lester/apps/rust_projects/axum-blog/blog.db";
const SQLITE_DSN:&'static str = "sqlite://G:/code/github/axum-blog/blog.db";


pub async fn init_db(){
    DB.get_or_init(|| async {
        let mut option = sea_orm::ConnectOptions::new(SQLITE_DSN.to_owned());
        option.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);

        Database::connect(option).await.expect("数据库连接失败")
    }).await;
    
} 

pub fn get_db<'a>()->Result<&'a DatabaseConnection>{
    let res = DB.get().ok_or(MyError::Default)?;
    Ok(res)
}
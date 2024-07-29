use tokio::sync::OnceCell;
use sea_orm::{DatabaseConnection,Database};
use std::time::Duration;

use crate::{MyError,Result, CONFIG};


pub static DB:OnceCell<DatabaseConnection> = OnceCell::const_new();


pub async fn init_db(){
    DB.get_or_init(|| async {
        let mut option = sea_orm::ConnectOptions::new(&CONFIG.database.dsn);
        option.max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false);

        Database::connect(option).await.expect("数据库连接失败")
    }).await;
    
} 
#[inline]
pub fn get_db<'a>()->Result<&'a DatabaseConnection>{
    let res = DB.get().ok_or(MyError::Default)?;
    Ok(res)
}
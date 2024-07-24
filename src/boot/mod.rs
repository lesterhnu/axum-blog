use tracing_appender::non_blocking::WorkerGuard;
use crate::Result;
pub mod config;
pub mod db;
pub mod logs;


pub async fn init()->Result<WorkerGuard>{
    db::init_db().await;
    let g = logs::register_log()?;
    Ok(g)
}
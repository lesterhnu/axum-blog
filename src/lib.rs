pub mod app;
pub mod boot;
pub mod error;
pub mod models;
pub mod routers;
pub mod resp; 

pub use app::*;
pub use boot::config::CONFIG;

pub use boot::db::get_db;
pub use error::MyError;

pub type Result<T> = std::result::Result<T, MyError>;


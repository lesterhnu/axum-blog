pub mod boot;
pub mod error;
pub mod handler;
pub mod models;
pub mod routers;
pub mod resp; 

pub use handler::posts;
pub use error::MyError;

pub type Result<T> = std::result::Result<T, MyError>;


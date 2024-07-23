mod routers;
mod handler;
use axum_blog::{Result};

#[tokio::main]
async fn main() {
    let app = routers::init_routers();

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(tcp_listener, app).await.expect("启动失败");
}

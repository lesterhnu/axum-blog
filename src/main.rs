use axum_blog::{boot, routers};
use tokio::signal;


#[tokio::main]
async fn main() {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_blog=debug,sea_orm=debug");
    }
    let _guard = boot::init().await;
    let app = routers::init_routers();
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(tcp_listener, app)
    .with_graceful_shutdown(shut_down())
    .await.expect("启动失败");
}

async fn shut_down() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::warn!("ctrl+c received, shutting down");
        },
        _ = terminate => {},
    }
}
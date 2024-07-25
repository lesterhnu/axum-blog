use askama_axum::Template;
use axum::{response::Html, routing::get, Router};

#[derive(Debug, Template)]
#[template(path = "../examples/template-demo/templates/index.html")]
pub struct IndexTemplate {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(tcp_listener, app).await.expect("启动失败");
}

async fn index() -> Html<String> {
    let index = IndexTemplate {
        name: "World".to_string(),
    };
    let s = index.render().unwrap().into();
    s
}

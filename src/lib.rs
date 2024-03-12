use axum::{http, routing::get, Router};
use tokio::net::TcpListener;

pub fn new_app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

pub async fn run() {
    tracing_subscriber::fmt::init();

    let app = new_app();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C signal handler");
}

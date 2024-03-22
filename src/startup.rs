use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::routes;

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe))
}

pub async fn run() {
    tracing_subscriber::fmt::init();

    let app = app();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C signal handler");
}

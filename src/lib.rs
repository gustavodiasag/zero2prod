use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};
use serde;
use tokio::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Subscription {
    email: String,
    name: String,
}

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
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

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(_subscription): Form<Subscription>) -> StatusCode {
    StatusCode::OK
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C signal handler");
}

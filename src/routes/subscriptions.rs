use axum::{http::StatusCode, Form};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Subscription {
    email: String,
    name: String,
}

pub async fn subscribe(Form(_subscription): Form<Subscription>) -> StatusCode {
    StatusCode::OK
}

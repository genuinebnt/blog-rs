use axum::{Form, response::IntoResponse};
use reqwest::StatusCode;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(Form(form_data): Form<FormData>) -> impl IntoResponse {
    StatusCode::OK
}

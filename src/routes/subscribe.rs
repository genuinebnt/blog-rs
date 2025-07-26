use crate::startup::AppState;
use axum::{Form, extract::State, response::IntoResponse, http::HeaderMap};
use reqwest::StatusCode;
use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Form(form_data): Form<FormData>,
) -> impl IntoResponse {
    let request_id = headers
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");

    tracing::info!(request_id = %request_id, "Received subscription request: {:?}", form_data);
    // Insert the subscriber into the database
    match sqlx::query!(
        "INSERT INTO users (id, email, name, password, created_at) VALUES ($1, $2, $3, $4, $5)",
        uuid::Uuid::new_v4(),
        form_data.email,
        form_data.name,
        "temporary_password", // In a real app, this would be hashed
        chrono::Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => tracing::info!("Subscriber added: {}", form_data.email),
        Err(e) => {
            tracing::error!("Failed to add subscriber: {}", e);
            let mut error_response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
            error_response.headers_mut().insert("x-request-id", request_id.parse().unwrap());
            return error_response;
        }
    };

    let mut response = StatusCode::OK.into_response();
    response.headers_mut().insert("x-request-id", request_id.parse().unwrap());
    response
}

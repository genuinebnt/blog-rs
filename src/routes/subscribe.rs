use crate::startup::AppState;
use axum::{Form, extract::State, http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a subscriber",
    skip(form_data, state, headers),
    fields(
        request_id = %headers.get("x-request-id").and_then(|v| v.to_str().ok()).unwrap_or("unknown"),
        subscriber_email = %form_data.email,
        subscriber_name = %form_data.name
    )
)]
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
        "INSERT INTO subscriptions(id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        uuid::Uuid::new_v4(),
        form_data.email,
        form_data.name,
        chrono::Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => tracing::info!("Subscriber added: {}", form_data.email),
        Err(e) => {
            tracing::error!("Failed to add subscriber: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut response = StatusCode::OK.into_response();
    response
        .headers_mut()
        .insert("x-request-id", request_id.parse().unwrap());
    response
}

use crate::startup::AppState;
use axum::{Form, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use std::sync::Arc;

#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Form(form_data): Form<FormData>,
) -> impl IntoResponse {
    // Insert the subscriber into the database
    let _result = sqlx::query!(
        "INSERT INTO users (id, email, name, password, created_at) VALUES ($1, $2, $3, $4, $5)",
        uuid::Uuid::new_v4(),
        form_data.email,
        form_data.name,
        "temporary_password", // In a real app, this would be hashed
        chrono::Utc::now()
    )
    .execute(&state.db_pool)
    .await;

    StatusCode::OK
}

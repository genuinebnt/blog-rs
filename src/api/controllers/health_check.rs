use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::containers::AppState;

pub async fn health_check(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "ok",
        "message": "Service is healthy",
    });
    (StatusCode::OK, Json(response))
}

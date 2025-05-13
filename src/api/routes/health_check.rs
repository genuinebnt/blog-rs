use axum::{Router, routing::get};

use crate::api::controllers::health_check::health_check;

use crate::containers::AppState;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(health_check))
        .with_state(state)
}

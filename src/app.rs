use std::sync::Arc;

use axum::Router;

use crate::{api::routes::health_check, containers::AppState};

pub fn create_app(state: Arc<AppState>) -> Router {
    Router::new().nest("/health_check", health_check::router(state))
}

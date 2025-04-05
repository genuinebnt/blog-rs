use axum::{Router, routing::get};

use super::handlers;

pub fn router() -> Router {
    Router::new().route("/", get(handlers::health_check))
}

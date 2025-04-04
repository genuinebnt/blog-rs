use axum::{Router, routing::get};

use crate::routes::health_check::health_check;

pub fn app() -> Router {
    axum::Router::new().route("/health_check", get(health_check))
}

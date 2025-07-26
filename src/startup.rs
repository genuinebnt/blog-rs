use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::{health_check::health_check, subscribe::subscribe};

pub async fn router() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
}

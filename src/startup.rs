use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use http;
use sqlx::postgres::PgConnectOptions;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::info_span;
use uuid::Uuid;

use crate::routes::{health_check::health_check, subscribe::subscribe};

#[derive(Debug)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

pub async fn router(opt: PgConnectOptions) -> Router {
    let db_pool = sqlx::PgPool::connect_with(opt)
        .await
        .expect("Failed to connect to the database");
    let app_state = Arc::new(AppState { db_pool });

    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscribe", post(subscribe))
        .layer(
            ServiceBuilder::new().layer(middleware::from_fn(crate::middleware::trace_middleware)),
        )
        .with_state(app_state)
}

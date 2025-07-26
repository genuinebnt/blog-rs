use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use http;
use sqlx::postgres::PgConnectOptions;
use tower_http::{
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::routes::{health_check::health_check, subscribe::subscribe};

#[derive(Debug)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
}

#[derive(Clone, Default)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().ok()?;
        Some(RequestId::new(request_id))
    }
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
            TraceLayer::new_for_http()
                .make_span_with(|request: &http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get("x-request-id")
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("unknown");

                    tracing::info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri = ?request.uri(),
                        request_id = %request_id,
                    )
                })
                .on_request(|_request: &http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("started processing request")
                })
                .on_response(
                    |_response: &http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(latency = ?latency, "finished processing request")
                    },
                ),
        )
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
        .layer(PropagateRequestIdLayer::x_request_id())
        .with_state(app_state)
}

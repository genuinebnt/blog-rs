use axum::response::IntoResponse;
use tracing::{Instrument, Span};

pub async fn trace_middleware(
    mut req: http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();

    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    req.headers_mut()
        .insert("x-request-id", request_id.parse().unwrap());

    let span = tracing::info_span!(
        "http request",
        method = %method,
        uri = %uri,
        version = ?version,
        status_code = tracing::field::Empty,
        response_time = tracing::field::Empty
    );

    async move {
        let start = std::time::Instant::now();
        let mut response = next.run(req).await;
        let elapsed = start.elapsed();

        Span::current().record("status_code", &response.status().as_u16());
        Span::current().record("response_time", &elapsed.as_millis());

        response
            .headers_mut()
            .insert("x-request-id", request_id.parse().unwrap());

        response
    }
    .instrument(span)
    .await
}

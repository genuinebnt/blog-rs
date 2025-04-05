use axum::Router;
use tokio::net::TcpListener;

use crate::api;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub async fn serve(&self, listener: TcpListener) {
        let router = Router::new()
            .nest("/v1/health_check", api::health_check::routes::router())
            .nest("/v1/posts", api::posts::routes::router());

        axum::serve(listener, router).await.unwrap();
    }
}

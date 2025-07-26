use axum_blog::startup::router;
use tokio::net::TcpListener;

pub async fn spawn_app() -> String {
    let router = router().await;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("{}:{}", "127.0.0.1", port);

    let _ = tokio::spawn(async { axum::serve(listener, router).await.unwrap() });

    address
}

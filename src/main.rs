use blog_rs::startup::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    let app = app();
    axum::serve(listener, app).await.unwrap();
}

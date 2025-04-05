use blog_rs::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    app::App::new().serve(listener).await;
}

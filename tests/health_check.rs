use tokio::net::TcpListener;

struct TestApp {
    address: String,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let router = blog_rs::startup::app();
    let _ = tokio::spawn(async move { axum::serve(listener, router).await });
    TestApp { address }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let app = spawn_app().await;

        let response = reqwest::get(app.address + "/health_check").await.unwrap();
        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }
}

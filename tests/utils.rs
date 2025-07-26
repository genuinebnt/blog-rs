use axum_blog::startup::router;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: sqlx::PgPool,
}

pub async fn spawn_app() -> TestApp {
    let settings = axum_blog::config::get_configuration().expect("Failed to read configuration");
    let router = router(settings.database.connect_options()).await;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("{}:{}", "127.0.0.1", port);

    let _ = tokio::spawn(async { axum::serve(listener, router).await.unwrap() });

    let db_pool = sqlx::PgPool::connect_with(settings.database.connect_options())
        .await
        .expect("Failed to connect to the database");

    TestApp { address, db_pool }
}

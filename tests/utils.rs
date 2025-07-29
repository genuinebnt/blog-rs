use axum_blog::startup::router;
use once_cell::sync::Lazy;
use tokio::net::TcpListener;

static LOGGER: Lazy<()> = Lazy::new(|| {
    axum_blog::logger::init_logger("test_app".to_string(), "info".to_string(), std::io::sink);
});

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: sqlx::PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&LOGGER);

    let settings = axum_blog::config::get_configuration().expect("Failed to read configuration");
    let router = router(settings.database.connect_options()).await;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let address = format!("{}:{}", "127.0.0.1", port);

    tokio::spawn(async { axum::serve(listener, router).await.unwrap() });

    let db_pool = sqlx::PgPool::connect_with(settings.database.connect_options())
        .await
        .expect("Failed to connect to the database");

    TestApp { address, db_pool }
}

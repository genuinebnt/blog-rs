use axum_blog::startup;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    axum_blog::logger::init_logger("blog".to_string(), "info".to_string(), std::io::stdout);
    let settings = axum_blog::config::get_configuration().expect("Failed to read configuration");
    tracing::info!("Configuration loaded: {:?}", settings);
    tracing::info!(
        "Starting server on {}:{}",
        settings.application.host,
        settings.application.port
    );
    let listener = TcpListener::bind(format!(
        "{}:{}",
        settings.application.host, settings.application.port
    ))
    .await
    .unwrap();
    let router = startup::router(settings.database.connect_options()).await;
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

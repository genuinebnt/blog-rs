use axum_blog::startup;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = axum_blog::config::get_configuration().expect("Failed to read configuration");
    println!(
        "Starting server on {}:{}",
        config.application.host, config.application.port
    );
    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .unwrap();
    let router = startup::router().await;
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

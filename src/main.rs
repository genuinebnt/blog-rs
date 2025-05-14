use blog_rs::{app::create_app, containers::AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let settings = blog_rs::config::Settings::new().expect("Failed to load settings");
    let connection_string =
        blog_rs::config::DatabaseSettings::connection_string_with_db(&settings.database);

    let state = AppState::build(connection_string).expect("Failed to build app state");
    let app = create_app(state);

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

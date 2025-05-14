use blog_rs::{app, config::Settings, containers::AppState};
use sqlx::types::Uuid;
use tokio::net::TcpListener;

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
}

pub async fn configure_database() -> anyhow::Result<Settings> {
    let mut settings = blog_rs::config::Settings::new()?;
    let connection_string =
        blog_rs::config::DatabaseSettings::connection_string_without_db(&settings.database);

    let db_pool = sqlx::PgPool::connect_lazy(&connection_string)
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;

    let database_name = Uuid::new_v4().to_string();
    let create_db_query = format!(r#"CREATE DATABASE "{}";"#, database_name);
    sqlx::query(&create_db_query)
        .execute(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create database: {}", e))?;

    settings.database.database_name = database_name;
    Ok(settings)
}

pub async fn spawn_app() -> TestApp {
    let settings = configure_database()
        .await
        .expect("Failed to configure database");

    let app_state = AppState::build(settings.database.connection_string_with_db())
        .expect("Failed to build app state");
    let app = app::create_app(app_state);

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind TCP listener");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let _ = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("Failed to start server");
    });

    TestApp { address }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use reqwest::Client;

    #[tokio::test]
    async fn health_check_works() {
        let app = spawn_app().await;
        let client = Client::new();
        let response = client
            .get(format!("{}/health_check", app.address))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::OK);
    }
}

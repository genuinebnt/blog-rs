use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::env;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connect_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .database(&self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Determine the environment
    let env = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let environment: Environment = env.as_str().try_into().expect("Invalid environment");

    // Load the .env file based on environment
    let env_file = format!(".env.{}", environment.as_str());
    if std::path::Path::new(&env_file).exists() {
        dotenvy::from_filename(&env_file).ok();
        tracing::info!("📋 Loaded environment from: {}", env_file);
    } else {
        // Fallback to default .env file
        dotenvy::dotenv().ok();
        tracing::info!("📋 Loaded environment from: .env (fallback)");
    }

    let settings = config::Config::builder()
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<&str> for Environment {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err("Invalid environment"),
        }
    }
}

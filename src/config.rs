use config::Config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub app_settings: AppSettings,
    pub database_settings: DatabaseSettings,
}

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir()
            .expect("Failed to get current directory")
            .join("config");

        let configuration_directory = base_path.join("configuration");
        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .try_into()
            .expect("Failed to parse environment");
        let environment_file_name = format!("{}.yaml", environment);
        let settings = Config::builder()
            .add_source(config::File::from(
                configuration_directory.join("base.yaml"),
            ))
            .add_source(config::File::from(
                configuration_directory.join(environment_file_name),
            ))
            .build()?;
        settings.try_deserialize()
    }
}

impl DatabaseSettings {
    pub fn connection_string_with_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub enum Environment {
    Development,
    Production,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Invalid environment: {}", value)),
        }
    }
}

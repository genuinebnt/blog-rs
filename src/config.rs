use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
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

pub fn get_configuration() -> Result<Config, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory");
    let config_path = base_path.join("config");

    let environment =
        std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let config_file = format!("{}.yaml", environment);
    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yaml")))
        .add_source(config::File::from(config_path.join(&config_file)))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    settings.try_deserialize::<Config>()
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!("Invalid environment: {}", value)),
        }
    }
}

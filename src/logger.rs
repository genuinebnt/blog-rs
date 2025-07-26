use crate::config::Environment;

pub fn init_logger() {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string())
        .as_str()
        .try_into()
        .unwrap();

    match environment {
        Environment::Development => {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(true)
                //.pretty()
                .init();
        }
        Environment::Production => {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_ansi(false)
                .json()
                .init();
        }
    };
}

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{
    EnvFilter, fmt::MakeWriter, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::config::Environment;

pub fn init_logger<Sink>(name: String, env_filter: String, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string())
        .as_str()
        .try_into()
        .unwrap();

    match environment {
        Environment::Development => tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_writer(sink)
            .with_ansi(true)
            .with_target(true)
            .init(),
        Environment::Production => {
            let env_filter =
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
            let formatting_layer = BunyanFormattingLayer::new(name, sink);

            tracing_subscriber::registry()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(formatting_layer)
                .init();
        }
    }
}

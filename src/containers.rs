use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn build() -> anyhow::Result<Arc<Self>> {
        Ok(Arc::new(AppState {}))
    }
}

use std::sync::Arc;

use sqlx::PgPool;
#[derive(Debug)]
pub struct AppState {
    db_pool: PgPool,
}

impl AppState {
    pub fn build(connection_string: String) -> anyhow::Result<Arc<Self>> {
        let db_pool = PgPool::connect_lazy(&connection_string)
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
        Ok(Arc::new(AppState { db_pool }))
    }
}

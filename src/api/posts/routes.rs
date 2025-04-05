use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::list_posts))
        .route("/", post(handlers::create_post))
        .route("/{id}", get(handlers::show_post))
        .route("/{id}", put(handlers::edit_post))
        .route("/{id}", delete(handlers::delete_post))
}

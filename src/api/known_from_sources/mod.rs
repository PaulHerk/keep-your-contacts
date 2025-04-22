pub mod known_from_sources_routes;

use axum::{
    routing::{get, put},
    Router,
};
use known_from_sources_routes::{
    delete_known_from_source, get_known_from_sources, update_known_from_source,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::MyState;

#[derive(Deserialize, Serialize, FromRow, Default, Clone)]
pub struct KnownFromSources {
    pub source_id: i32,
    pub source_name: String,
    pub description: String,
}

pub fn create_known_from_sources_router() -> Router<MyState> {
    Router::new()
        .route("/known-from-sources", get(get_known_from_sources))
        .route(
            "/known-from-sources/{source_id}",
            put(update_known_from_source).delete(delete_known_from_source),
        )
}

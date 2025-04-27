pub mod known_from_sources_routes;

use axum::{
    routing::{get, put},
    Router,
};
use known_from_sources_routes::{
    create_known_from_source, delete_known_from_source, get_known_from_sources,
    update_known_from_source,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::MyState;

#[derive(Deserialize, Serialize, FromRow, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KnownFromSources {
    // user_id is omitted since frontend doesn't need this. Is that how you build APIs? Idk
    pub source_id: i32,
    pub source_name: String,
    pub description: String,
}
#[derive(Deserialize, Serialize, FromRow, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewKnownFromSources {
    pub source_name: String,
    pub description: String,
}

pub fn create_known_from_sources_router() -> Router<MyState> {
    Router::new()
        .route(
            "/known-from-sources",
            get(get_known_from_sources).post(create_known_from_source),
        )
        .route(
            "/known-from-sources/{source_id}",
            put(update_known_from_source).delete(delete_known_from_source),
        )
}

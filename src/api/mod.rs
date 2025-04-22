use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use sqlx::{
    types::{
        chrono::{DateTime, Local},
        Json,
    },
    FromRow, PgPool,
};

pub mod auth;
pub mod errors;
pub mod known_from_sources_routes;
pub mod post_person;
pub mod retrieve_persons;
pub mod update_person;

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
    pub secrets: SecretStore,
}

#[derive(Serialize, Deserialize, Default, FromRow, Copy, Clone)]
pub struct Coordinate {
    pub lon: f64,
    pub lat: f64,
}

#[derive(FromRow, Default, Serialize, Deserialize, Clone)]

pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub known_from_source_id: Option<i32>,
    #[serde(skip_serializing)]
    pub coordinate: Option<Json<Coordinate>>,
    pub job_title: String,
    pub company: String,
    pub linkedin: String,
    pub notes: String,
    pub created_at: DateTime<Local>, // pub born: String,
}
trait PersonTrait {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>>;
}
impl PersonTrait for Person {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}

#[derive(Deserialize, Serialize, FromRow, Default, Clone)]
pub struct KnownFromSources {
    pub source_id: i32,
    pub source_name: String,
    pub description: String,
}

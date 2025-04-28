mod get_coordinates;
mod post_person;
pub mod retrieve_persons;
mod update_person;

use axum::{
    routing::{get, put},
    Router,
};
use chrono::{DateTime, Local};
use get_coordinates::get_persons_with_coords;
use post_person::create_person;
use retrieve_persons::{
    get_person_count::get_person_count,
    get_persons::{get_single_person, retrieve},
};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use update_person::{delete_person, update_person};

use super::MyState;

#[derive(Serialize, Deserialize, Default, FromRow, Copy, Clone, Debug)]
pub struct Coordinate {
    pub lon: f64,
    pub lat: f64,
}

#[derive(FromRow, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
pub trait PersonTrait {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>>;
}
impl PersonTrait for Person {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}

pub fn create_persons_router() -> Router<MyState> {
    Router::new()
        .route("/persons", get(retrieve).post(create_person))
        .route("/persons/count", get(get_person_count))
        .route("/persons/coordinates", get(get_persons_with_coords))
        .route(
            "/persons/{person_id}",
            put(update_person)
                .delete(delete_person)
                .get(get_single_person),
        )
}

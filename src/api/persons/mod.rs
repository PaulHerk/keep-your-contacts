mod get_coordinates;
pub mod retrieve_persons;
mod update_person;

use axum::{
    routing::{get, put},
    Router,
};
use chrono::{DateTime, Local};
use get_coordinates::get_persons_with_coords;
use retrieve_persons::{
    get_person_count::get_person_count,
    get_persons::{get_single_person, retrieve},
};
use reverse_geocoder::{Record, ReverseGeocoder};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};
use update_person::create_person;
use update_person::{delete_person, update_person};

use super::MyState;
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Default, FromRow, Clone, Debug)]
pub struct CoordinateSearch {
    pub search: String,
    pub lon: f64,
    pub lat: f64,
}

#[derive(FromRow, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub known_from_source_id: Option<i32>,
    #[serde(skip_serializing)]
    pub coordinate_with_search: Option<sqlx::types::Json<CoordinateSearch>>,
    pub job_title: String,
    pub company: String,
    pub linkedin: String,
    pub notes: String,
    pub created_at: DateTime<Local>, // pub born: String,
}

/// needed for the coordinate record that gets appened to the response
#[derive(Deserialize, Serialize)]
pub struct UserResponse<Fetched> {
    #[serde(flatten)]
    pub person: Fetched,
    pub record: Option<Record>,
}

pub fn get_record_from_coord(
    geocoder: &ReverseGeocoder,
    coord: Option<Json<CoordinateSearch>>,
) -> Option<Record> {
    if let Some(coord) = coord {
        Some(geocoder.search((coord.lat, coord.lon)).record).cloned()
    } else {
        None
    }
}
fn create_person_with_record<Person: PersonTrait + Clone>(
    persons: Vec<Person>,
    geocoder: &ReverseGeocoder,
) -> Vec<UserResponse<Person>> {
    persons
        .iter()
        .map(|person| {
            let record = get_record_from_coord(geocoder, person.get_coord());
            let fetched = person.clone();
            UserResponse {
                person: fetched,
                record,
            }
        })
        .collect()
}

pub trait PersonTrait {
    fn get_coord(&self) -> Option<Json<CoordinateSearch>>;
}
impl PersonTrait for Person {
    fn get_coord(&self) -> Option<Json<CoordinateSearch>> {
        self.coordinate_with_search.clone() // TODO: theoretically the search query is not needed,
                                            // which would remove the .clone()
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

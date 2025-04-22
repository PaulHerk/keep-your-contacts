use reverse_geocoder::Record;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{Coordinate, Person, PersonTrait};

pub mod filter_persons;
pub mod get_persons;

#[derive(Deserialize, Clone, Debug)]
pub struct PaginationFilterQuery {
    pub page: i32,
    pub per_page: i32,
    pub detailed: Option<bool>,
    #[serde(flatten)]
    pub filter: Filtering,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Filtering {
    pub global_search: Option<String>,
    pub known_from_search: Option<i32>,
}

enum UserView {
    Simple,
    Detailed,
}

/// two different views: simple and detailed, SimplePerson is just names and coords
#[derive(Deserialize, FromRow, Serialize, Clone)]
pub struct SimplePerson {
    id: i32,
    first_name: String,
    #[sqlx(default)]
    last_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing)]
    coordinate: Option<sqlx::types::Json<Coordinate>>,
}

impl PersonTrait for SimplePerson {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}
/// needed for the coordinate record that gets appened to the response
#[derive(Deserialize, Serialize)]
pub struct UserResponse<Fetched> {
    #[serde(flatten)]
    person: Fetched,
    record: Option<Record>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserQueryResult {
    Simple(Vec<UserResponse<SimplePerson>>),
    Detailed(Vec<UserResponse<Person>>),
}

use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};

use super::{CoordinateSearch, Person, PersonTrait, UserResponse};

pub mod filter_persons;
pub mod get_person_count;
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
    pub known_from_search: Option<String>,
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
    birthday: Option<chrono::NaiveDate>,
    #[serde(skip_serializing)]
    coordinate_with_search: Option<Json<CoordinateSearch>>,
}

impl PersonTrait for SimplePerson {
    fn get_coord(&self) -> Option<Json<CoordinateSearch>> {
        self.coordinate_with_search.clone()
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserQueryResult {
    Simple(Vec<UserResponse<SimplePerson>>),
    Detailed(Vec<UserResponse<Person>>),
}

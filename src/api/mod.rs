use serde::{Deserialize, Serialize};
use sqlx::{
    types::{
        chrono::{DateTime, Local},
        Json,
    },
    FromRow, PgPool,
};

pub mod get_persons;
pub mod post_person;

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
}

#[derive(Deserialize, Default)]
pub struct PersonNew {
    pub name: String,
    pub known_from_source_id: Option<i32>,
    pub coordinate: Option<Coordinate>,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub linkedin: Option<String>,
    pub notes: Option<String>,
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
    pub last_name: Option<String>,
    pub known_from_source_id: Option<i32>,
    #[serde(skip_serializing)]
    pub coordinate: Option<Json<Coordinate>>,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub linkedin: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Local>>, // pub born: String,
}

#[derive(Serialize, FromRow, Default)]
pub struct KnownFromSources {
    pub source_id: i32,
    pub source_name: String,
    pub description: String,
}

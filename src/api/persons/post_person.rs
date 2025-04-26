use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use geocoding::{
    openstreetmap::{OpenstreetmapParams, OpenstreetmapResponse},
    Openstreetmap,
};
use serde::Deserialize;
use sqlx::{Postgres, QueryBuilder};

use crate::api::{auth::User, errors::Error, MyState};

use super::{Coordinate, Person};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CoordOrOSMSearch {
    Coordinate(Coordinate),
    OSMSearch(String),
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersonNew {
    pub name: String,
    pub known_from_source_id: Option<i32>,
    pub coordinate_or_osm_search: Option<CoordOrOSMSearch>,
    #[serde(default)]
    pub job_title: String,
    #[serde(default)]
    pub company: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub notes: String,
}

pub async fn create_person(
    State(state): State<MyState>,
    Extension(user): Extension<User>,
    Json(data): Json<PersonNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO persons (user_id, first_name, last_name, known_from_source_id, coordinate, job_title, company, linkedin, notes) VALUES (",
    );

    let mut names = data.name.trim().split_ascii_whitespace();
    let first_name = names.next().unwrap_or_default();
    let last_name: String = names.collect::<Vec<&str>>().join(" ");

    dbg!(&data.coordinate_or_osm_search);

    let coords = if let Some(coord_or_osm_search) = data.coordinate_or_osm_search {
        match coord_or_osm_search {
            CoordOrOSMSearch::Coordinate(coordinate) => Some(coordinate),
            CoordOrOSMSearch::OSMSearch(search_query) => {
                let osm = Openstreetmap::new();
                let params = OpenstreetmapParams::new(&search_query);
                let result: Result<OpenstreetmapResponse<f64>, _> = osm.forward_full(&params);
                match result {
                    Ok(result) => {
                        let (lat, lon) = result.features[0].geometry.coordinates;
                        let coordinate = Coordinate { lat, lon };
                        Some(coordinate)
                    }
                    Err(error) => return Err(Error::CityNotFound(error)),
                }
            }
        }
    } else {
        None
    };

    let mut field_separator = query_builder.separated(", ");
    field_separator
        .push_bind(user.id)
        .push_bind(first_name.trim())
        .push_bind(last_name.trim())
        .push_bind(data.known_from_source_id)
        .push_bind(coords.map(|coordinate| serde_json::json!(coordinate)))
        .push_bind(data.job_title)
        .push_bind(data.company)
        .push_bind(data.linkedin)
        .push_bind(data.notes);
    field_separator.push_unseparated(") RETURNING *;");

    match query_builder
        .build_query_as::<Person>()
        .fetch_one(&state.pool)
        .await
    {
        Ok(person) => Ok((StatusCode::CREATED, Json(person))),
        Err(e) => Err(Error::DBError(e)),
    }
}

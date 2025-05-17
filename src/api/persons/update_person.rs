use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;

use crate::api::{auth::UserWithSettings, errors::Error, persons::PlaceRecord, MyState};

use super::{CoordinateSearch, Person};
use reverse_geocoder::ReverseGeocoder;
use sqlx::{Postgres, QueryBuilder};

use crate::api::persons::{get_record_from_coord, UserResponse};

#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PersonNew {
    pub name: String,
    pub known_from_source_id: Option<i32>,
    pub coordinate_with_search: Option<CoordinateSearch>,
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
    Extension(user): Extension<UserWithSettings>,
    Json(data): Json<PersonNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO persons (user_id, first_name, last_name, known_from_source_id, coordinate_with_search, job_title, company, linkedin, notes) VALUES (",
    );

    let mut names = data.name.trim().split_ascii_whitespace();
    let first_name = names.next().unwrap_or_default();
    let last_name: String = names.collect::<Vec<&str>>().join(" ");

    let mut field_separator = query_builder.separated(", ");
    field_separator
        .push_bind(user.user.id)
        .push_bind(first_name.trim())
        .push_bind(last_name.trim())
        .push_bind(data.known_from_source_id)
        .push_bind(data.coordinate_with_search.map(|c| serde_json::json!(c)))
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
        Ok(person) => {
            let geocoder = ReverseGeocoder::new();
            let record = get_record_from_coord(&geocoder, &person.coordinate_with_search);
            Ok(Json(UserResponse {
                person: person.clone(),
                record: PlaceRecord::from_coord_and_record(&person.coordinate_with_search, record),
            }))
        }
        Err(e) => Err(Error::DBError(e)),
    }
}

pub async fn update_person(
    State(state): State<MyState>,
    Extension(user): Extension<UserWithSettings>,
    Path(person_id): Path<i32>,
    Json(data): Json<PersonNew>,
) -> Result<impl IntoResponse, Error> {
    let mut names = data.name.trim().split_ascii_whitespace();
    let first_name = names.next().unwrap_or_default();
    let last_name: String = names.collect::<Vec<&str>>().join(" ");

    let person: Person = sqlx::query_as("UPDATE Persons SET (first_name, last_name, known_from_source_id, coordinate_with_search, job_title, company, linkedin, notes) = ($3, $4, $5, $6, $7, $8, $9, $10)  WHERE user_id = $1 AND id = $2 RETURNING *")
        .bind(user.user.id)
        .bind(person_id)
        .bind(first_name)
        .bind(last_name)
        .bind(data.known_from_source_id)
        .bind(data.coordinate_with_search.map(|coord| serde_json::json!(coord)))
        .bind(data.job_title)
        .bind(data.company)
        .bind(data.linkedin)
        .bind(data.notes).fetch_one(&state.pool).await.map_err(Error::DBError)?;
    Ok((StatusCode::CREATED, Json(person)))
}

pub async fn delete_person(
    State(state): State<MyState>,
    Extension(user): Extension<UserWithSettings>,
    Path(person_id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let rows_affected = sqlx::query("DELETE FROM Persons WHERE user_id = $1 AND id = $2")
        .bind(user.user.id)
        .bind(person_id)
        .execute(&state.pool)
        .await
        .map_err(Error::DBError)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::PersonNotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

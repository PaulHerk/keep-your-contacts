use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

pub mod get_metadata;

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct PersonNew {
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub note: String,
}

#[derive(Serialize, FromRow, Default)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub note: String,
    // pub born: String,
}

pub async fn retrieve(
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Person>("SELECT * FROM persons")
        .fetch_all(&state.pool)
        .await
    {
        Ok(person) => Ok((StatusCode::OK, Json(person))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn add_person(
    State(state): State<MyState>,
    Json(data): Json<PersonNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Person>(
        "INSERT INTO persons (note, first_name, last_name, city) VALUES ($1, $2, $3, $4) RETURNING id, note, first_name, last_name, city",
    )
    .bind(&data.note)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.city)
    .fetch_one(&state.pool)
    .await
    {
        Ok(person) => Ok((StatusCode::CREATED, Json(person))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

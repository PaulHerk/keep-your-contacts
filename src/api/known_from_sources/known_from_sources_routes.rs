use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;

use crate::api::{auth::User, errors::Error, MyState};

use super::{KnownFromSources, NewKnownFromSources};

pub async fn create_known_from_source(
    Extension(user): Extension<User>,
    State(state): State<MyState>,
    Json(known_from_source): Json<NewKnownFromSources>,
) -> Result<impl IntoResponse, Error> {
    let id: i32 = sqlx::query_scalar(
        "INSERT INTO KnownFromSources (user_id, source_name, description) VALUES ($1, $2, $3) RETURNING source_id",
    )
    .bind(user.id)
    .bind(known_from_source.source_name)
    .bind(known_from_source.description)
    .fetch_one(&state.pool)
    .await
    .map_err(Error::DBError)?;

    Ok((StatusCode::CREATED, Json(id)))
}

pub async fn get_known_from_sources(
    Extension(user): Extension<User>,
    State(state): State<MyState>,
) -> Result<Json<Vec<KnownFromSources>>, Error> {
    let known_from_sources = sqlx::query_as::<_, KnownFromSources>(
        "SELECT source_id, source_name, description FROM KnownFromSources WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(Error::DBError)?;

    Ok(Json(known_from_sources))
}

#[derive(Deserialize)]
pub struct UpdateKnownFromSource {
    source_name: Option<String>,
    description: Option<String>,
}

pub async fn update_known_from_source(
    Extension(user): Extension<User>,
    State(state): State<MyState>,
    Path(source_id): Path<i32>,
    Json(known_from_source): Json<UpdateKnownFromSource>,
) -> Result<StatusCode, Error> {
    let row = sqlx::query_as::<_, KnownFromSources>(
        "SELECT * FROM KnownFromSources WHERE user_id = $1 AND source_id = $2",
    )
    .bind(user.id)
    .bind(source_id)
    .fetch_one(&state.pool)
    .await
    .map_err(Error::DBError)?;

    sqlx::query(
    "UPDATE KnownFromSources SET source_name = $1, description = $2 WHERE user_id = $3 AND source_id = $4",
)
.bind(known_from_source.source_name.unwrap_or(row.source_name))
.bind(known_from_source.description.unwrap_or(row.description))
.bind(user.id)
.bind(source_id)
.execute(&state.pool)
.await
.map_err(Error::DBError)?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_known_from_source(
    Extension(user): Extension<User>,
    State(state): State<MyState>,
    Path(source_id): Path<i32>,
) -> Result<StatusCode, Error> {
    let rows_effected =
        sqlx::query("DELETE FROM KnownFromSources WHERE user_id = $1 AND source_id = $2")
            .bind(user.id)
            .bind(source_id)
            .execute(&state.pool)
            .await
            .map_err(Error::DBError)?
            .rows_affected();

    if rows_effected == 0 {
        return Err(Error::KnownFromSourceNotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

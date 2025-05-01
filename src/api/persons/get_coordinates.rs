use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::api::{auth::User, errors::Error, MyState};

use super::Coordinate;

#[derive(FromRow, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MapContactResponse {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    coordinate: sqlx::types::Json<Coordinate>,
}

pub async fn get_persons_with_coords(
    State(state): State<MyState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let coordinates:Vec<MapContactResponse> = sqlx::query_as(
        "SELECT id, first_name, last_name, coordinate FROM persons WHERE user_id = $1 AND coordinate IS NOT NULL",
    )
    .bind(user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(Error::DBError)?;

    Ok(Json(coordinates))
}

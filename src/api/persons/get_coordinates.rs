use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::api::{auth::UserWithSettings, errors::Error, MyState};

use super::CoordinateSearch;

#[derive(FromRow, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct MapContactResponse {
    id: i32,
    first_name: String,
    last_name: Option<String>,
    coordinate_with_search: sqlx::types::Json<CoordinateSearch>,
}

pub async fn get_persons_with_coords(
    State(state): State<MyState>,
    Extension(user): Extension<UserWithSettings>,
) -> Result<impl IntoResponse, Error> {
    let coordinates = sqlx::query_as!( MapContactResponse, 
        r#"SELECT id, first_name, last_name, coordinate_with_search as "coordinate_with_search!: sqlx::types::Json<CoordinateSearch>" FROM persons WHERE user_id = $1 AND coordinate_with_search IS NOT NULL"#,
    user.user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(Error::DBError)?;

    Ok(Json(coordinates))
}

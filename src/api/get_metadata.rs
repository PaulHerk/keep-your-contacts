use super::{MyState, Person};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
struct Count {
    count: i32,
}

pub async fn get_metadata(
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let count = sqlx::query_as::<_, Count>("SELECT count(*) FROM persons")
        .fetch_one(&state.pool)
        .await;

    match count {
        Ok(count) => {
            return Ok((StatusCode::OK, Json(count)));
        }
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

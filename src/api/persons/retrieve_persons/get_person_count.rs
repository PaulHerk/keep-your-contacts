use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::api::{auth::UserWithSettings, errors::Error, MyState};

pub async fn get_person_count(
    State(state): State<MyState>,
    Extension(user): Extension<UserWithSettings>,
) -> Result<impl IntoResponse, Error> {
    let person_count = sqlx::query_scalar!(
        "SELECT count(id) FROM persons WHERE user_id = $1",
        user.user.id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(Error::DBError)?;

    Ok(Json(person_count))
}

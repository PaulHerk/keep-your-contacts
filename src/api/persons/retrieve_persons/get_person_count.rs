use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::api::{auth::User, errors::Error, MyState};

pub async fn get_person_count(
    State(state): State<MyState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, Error> {
    let person_count: i64 = sqlx::query_scalar("SELECT count(id) FROM persons WHERE user_id = $1")
        .bind(user.id)
        .fetch_one(&state.pool)
        .await
        .map_err(Error::DBError)?;

    Ok(Json(person_count))
}

use axum::{extract::State, Extension, Json};

use super::{auth::User, errors::Error, KnownFromSources, MyState};

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
    .map_err(|e| Error::DBError(e))?;

    Ok(Json(known_from_sources))
}

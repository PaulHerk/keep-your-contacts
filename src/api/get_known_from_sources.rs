use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use super::{auth::User, KnownFromSources, MyState};

pub async fn get_known_from_sources(
    Extension(user): Extension<User>,
    State(state): State<MyState>,
) -> Result<Json<Vec<KnownFromSources>>, (StatusCode, String)> {
    let known_from_sources = sqlx::query_as::<_, KnownFromSources>(
        "SELECT source_id, source_name, description FROM KnownFromSources WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?;

    Ok(Json(known_from_sources))
}

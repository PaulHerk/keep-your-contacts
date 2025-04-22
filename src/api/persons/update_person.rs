use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use crate::api::{auth::User, errors::Error, MyState};

use super::{Coordinate, Person};
#[derive(Serialize, Deserialize, Default)]
pub struct UpdatePerson {
    pub last_name: Option<String>,
    pub known_from_source_id: Option<i32>,
    pub coordinate: Option<Coordinate>,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub linkedin: Option<String>,
    pub notes: Option<String>,
}

pub async fn update_person(
    State(state): State<MyState>,
    Extension(user): Extension<User>,
    Path(person_id): Path<i32>,
    Json(data): Json<UpdatePerson>,
) -> Result<impl IntoResponse, Error> {
    let person =
        sqlx::query_as::<_, Person>("SELECT * FROM Persons WHERE user_id = $1 AND id = $2") // only id should be enough but I want to prevent any secret information to be leaked
            .bind(user.id)
            .bind(person_id)
            .fetch_one(&state.pool)
            .await
            .map_err(|e| Error::DBError(e))?;

    sqlx::query("UPDATE Persons SET (last_name, known_from_source_id, coordinate, job_title, company, linkedin, notes) = ($3, $4, $5, $6, $7, $8, $9)  WHERE user_id = $1 AND id = $2")
        .bind(user.id)
        .bind(person_id)
        .bind(data.last_name.unwrap_or(person.last_name))
        .bind(data.known_from_source_id.or(person.known_from_source_id))
        .bind(data.coordinate.or(person.coordinate.map(|p| p.0)).map(|coord| serde_json::json!(coord)))
        .bind(data.job_title.unwrap_or(person.job_title))
        .bind(data.company.unwrap_or(person.company))
        .bind(data.linkedin.unwrap_or(person.linkedin))
        .bind(data.notes.unwrap_or(person.notes)).execute(&state.pool).await.map_err(|e| Error::DBError(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn delete_person(
    State(state): State<MyState>,
    Extension(user): Extension<User>,
    Path(person_id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let rows_affected = sqlx::query("DELETE FROM Persons WHERE user_id = $1 AND id = $2")
        .bind(user.id)
        .bind(person_id)
        .execute(&state.pool)
        .await
        .map_err(|e| Error::DBError(e))?
        .rows_affected();

    if rows_affected == 0 {
        return Err(Error::PersonNotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

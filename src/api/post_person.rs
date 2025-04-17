use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::{Postgres, QueryBuilder};

use super::{MyState, PersonNew};

pub async fn add_person(
    State(state): State<MyState>,
    Json(data): Json<PersonNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "INSERT INTO persons (first_name, last_name, known_from_source_id, coordinate, job_title, company, linkedin, notes) VALUES (",
    );

    let mut names = data.name.split_ascii_whitespace();
    let first_name = names.next().unwrap_or_default();
    let last_name: String = names.collect::<Vec<&str>>().join(" ");

    let mut field_separator = query_builder.separated(", ");
    field_separator
        .push_bind(first_name)
        .push_bind(last_name)
        .push_bind(data.known_from_source_id)
        .push_bind(data.coordinate.map(|coord| serde_json::json!(&coord)))
        .push_bind(data.job_title)
        .push_bind(data.company)
        .push_bind(data.linkedin)
        .push_bind(data.notes);
    field_separator.push_unseparated(") RETURNING id;");

    #[derive(sqlx::FromRow)]
    struct InsertedId {
        id: i32,
    }

    match query_builder
        .build_query_as::<InsertedId>()
        .fetch_one(&state.pool)
        .await
    {
        Ok(inserted_id) => Ok((StatusCode::CREATED, Json(inserted_id.id))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

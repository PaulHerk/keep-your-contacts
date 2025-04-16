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

    let mut field_separator = query_builder.separated(", ");
    field_separator
        .push_bind(data.first_name)
        .push_bind(data.last_name)
        .push_bind(data.known_from_source_id)
        .push_bind(data.coordinate.map(|coord| serde_json::json!(&coord)))
        .push_bind(data.job_title)
        .push_bind(data.company)
        .push_bind(data.linkedin)
        .push_bind(data.notes);
    field_separator.push_unseparated(");");

    println!("{}", query_builder.sql());
    match query_builder.build().execute(&state.pool).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_geocoder::{Record, ReverseGeocoder};
use sqlx::{Postgres, QueryBuilder};

use super::*;

#[derive(Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub per_page: i32,
    pub detailed: Option<bool>,
}

enum UserView {
    Simple,
    Detailed,
}

#[derive(Deserialize, FromRow, Serialize, Clone)]
pub struct SimplePerson {
    id: i32,
    first_name: String,
    #[sqlx(default)]
    last_name: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing)]
    coordinate: Option<sqlx::types::Json<Coordinate>>,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponse<Fetched> {
    #[serde(flatten)]
    fetched: Fetched,
    #[serde(flatten)]
    record: Option<Record>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserQueryResult {
    Simple(Vec<UserResponse<SimplePerson>>),
    Detailed(Vec<UserResponse<Person>>),
}

trait PersonT {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>>;
}

impl PersonT for SimplePerson {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}
impl PersonT for Person {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}

fn get_record_from_coord(
    geocoder: &ReverseGeocoder,
    coord: Option<sqlx::types::Json<Coordinate>>,
) -> Option<Record> {
    return if let Some(coord) = coord {
        Some(geocoder.search((coord.langitude, coord.latitude)).record).cloned()
    } else {
        None
    };
}
fn create_person_with_record<Person: PersonT + Clone>(
    persons: Vec<Person>,
    geocoder: &ReverseGeocoder,
) -> Vec<UserResponse<Person>> {
    persons
        .iter()
        .map(|person| {
            let record = get_record_from_coord(&geocoder, person.get_coord());
            let fetched = person.clone();
            UserResponse { fetched, record }
        })
        .collect()
}

pub async fn retrieve(
    pagination: Query<Pagination>,
    State(state): State<MyState>,
) -> Result<(axum::http::StatusCode, Json<UserQueryResult>), impl IntoResponse> {
    let view = match pagination.detailed.unwrap_or(false) {
        true => UserView::Detailed,
        false => UserView::Simple,
    };
    let geocoder = ReverseGeocoder::new();
    let mut query_end_builder: QueryBuilder<Postgres> = QueryBuilder::new("FROM persons OFFSET");
    query_end_builder
        .push_bind(pagination.per_page * pagination.page)
        .push("LIMIT")
        .push_bind(pagination.per_page);
    let query_end = query_end_builder.sql();

    match view {
        UserView::Simple => {
            match sqlx::query_as::<_, SimplePerson>(&format!(
                "SELECT id, first_name, last_name, coordinate {query_end}",
            ))
            .fetch_all(&state.pool)
            .await
            {
                Ok(persons) => {
                    let simple_persons_with_coords = create_person_with_record(persons, &geocoder);
                    return Ok((
                        StatusCode::OK,
                        Json(UserQueryResult::Simple(simple_persons_with_coords)),
                    ));
                }
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        UserView::Detailed => {
            match sqlx::query_as::<_, Person>(&format!("SELECT * {query_end}"))
                .fetch_all(&state.pool)
                .await
            {
                Ok(persons) => {
                    let persons_with_coords = create_person_with_record(persons, &geocoder);
                    return Ok((
                        StatusCode::OK,
                        Json(UserQueryResult::Detailed(persons_with_coords)),
                    ));
                }
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
    }
}

// pub async fn add_person(
//     State(state): State<MyState>,
//     Json(data): Json<PersonNew>,
// ) -> Result<impl IntoResponse, impl IntoResponse> {
//     match sqlx::query_as::<_, Person>(
//         "INSERT INTO persons (first_name, last_name, city, job, note) VALUES ($1, $2, $3, $4, $5) RETURNING id, note, first_name, last_name, city, job",
//     )
//     .bind(&data.first_name)
//     .bind(&data.last_name)
//     .bind(&data.city)
//     .bind(&data.job)
//     .bind(&data.note)
//     .fetch_one(&state.pool)
//     .await
//     {
//         Ok(person) => Ok((StatusCode::CREATED, Json(person))),
//         Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
//     }
// }

// pub async fn delete_person(
//     Path(id): Path<i32>,
//     State(state): State<MyState>,
// ) -> Result<impl IntoResponse, impl IntoResponse> {
//     match sqlx::query("DELETE FROM persons WHERE id = $1")
//         .bind(id)
//         .execute(&state.pool)
//         .await
//     {
//         Ok(_) => Ok((StatusCode::OK, ())),
//         Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
//     }
// }

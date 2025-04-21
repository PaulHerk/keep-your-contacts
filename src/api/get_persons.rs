use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use reverse_geocoder::{Record, ReverseGeocoder};
use sqlx::{Postgres, QueryBuilder};

use super::{auth::User, *};

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

// two different views: simple and detailed, SimplePerson is just names and coords
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

// this is needed for the coordinate record that gets appened to the response
#[derive(Deserialize, Serialize)]
pub struct UserResponse<Fetched> {
    #[serde(flatten)]
    person: Fetched,
    #[serde(flatten)]
    record: Option<Record>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum UserQueryResult {
    Simple(Vec<UserResponse<SimplePerson>>),
    Detailed(Vec<UserResponse<Person>>),
}

impl PersonTrait for SimplePerson {
    fn get_coord(&self) -> Option<sqlx::types::Json<Coordinate>> {
        self.coordinate
    }
}

fn get_record_from_coord(
    geocoder: &ReverseGeocoder,
    coord: Option<sqlx::types::Json<Coordinate>>,
) -> Option<Record> {
    return if let Some(coord) = coord {
        Some(geocoder.search((coord.lon, coord.lat)).record).cloned()
    } else {
        None
    };
}
fn create_person_with_record<Person: PersonTrait + Clone>(
    persons: Vec<Person>,
    geocoder: &ReverseGeocoder,
) -> Vec<UserResponse<Person>> {
    persons
        .iter()
        .map(|person| {
            let record = get_record_from_coord(&geocoder, person.get_coord());
            let fetched = person.clone();
            UserResponse {
                person: fetched,
                record,
            }
        })
        .collect()
}

pub async fn retrieve(
    pagination: Query<Pagination>,
    Extension(user): Extension<User>,
    State(state): State<MyState>,
) -> Result<(StatusCode, Json<UserQueryResult>), (StatusCode, String)> {
    let view = match pagination.detailed.unwrap_or(false) {
        true => UserView::Detailed,
        false => UserView::Simple,
    };
    let geocoder = ReverseGeocoder::new();
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(match view {
        UserView::Simple => "SELECT id, first_name, last_name, coordinate",
        UserView::Detailed => "SELECT *",
    });
    query
        .push(" FROM persons WHERE user_id = ")
        .push_bind(user.id)
        .push(" OFFSET ")
        .push_bind(pagination.per_page * pagination.page)
        .push(" LIMIT ")
        .push_bind(pagination.per_page);

    match view {
        UserView::Simple => {
            let persons = query
                .build_query_as::<SimplePerson>()
                .fetch_all(&state.pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?;
            let simple_persons_with_coords = create_person_with_record(persons, &geocoder);
            Ok((
                StatusCode::OK,
                Json(UserQueryResult::Simple(simple_persons_with_coords)),
            ))
        }
        UserView::Detailed => {
            let persons = query
                .build_query_as::<Person>()
                .fetch_all(&state.pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?;
            let persons_with_coords = create_person_with_record(persons, &geocoder);
            Ok((
                StatusCode::OK,
                Json(UserQueryResult::Detailed(persons_with_coords)),
            ))
        }
    }
}

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

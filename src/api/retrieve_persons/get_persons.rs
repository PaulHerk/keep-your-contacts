use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use reverse_geocoder::{Record, ReverseGeocoder};
use sqlx::{Postgres, QueryBuilder};

use crate::api::{auth::User, Coordinate, MyState, Person, PersonTrait};

use super::{
    filter_persons::filter_person_query, PaginationFilterQuery, SimplePerson, UserQueryResult,
    UserResponse, UserView,
};

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
    Extension(user): Extension<User>,
    Query(url_query): Query<PaginationFilterQuery>,
    State(state): State<MyState>,
) -> Result<(StatusCode, Json<UserQueryResult>), (StatusCode, String)> {
    let view = match url_query.detailed.unwrap_or(false) {
        true => UserView::Detailed,
        false => UserView::Simple,
    };
    let geocoder = ReverseGeocoder::new();
    let mut sql_query: QueryBuilder<Postgres> = QueryBuilder::new(match view {
        UserView::Simple => "SELECT id, first_name, last_name, coordinate",
        UserView::Detailed => "SELECT *",
    });
    sql_query
        .push(" FROM persons WHERE user_id = ")
        .push_bind(user.id);

    sql_query = filter_person_query(sql_query, url_query.filter);
    sql_query
        .push(" OFFSET ")
        .push_bind(url_query.per_page * url_query.page)
        .push(" LIMIT ")
        .push_bind(url_query.per_page);
    match view {
        UserView::Simple => {
            let persons = sql_query
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
            let persons = sql_query
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

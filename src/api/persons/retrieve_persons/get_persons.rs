use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use reverse_geocoder::ReverseGeocoder;
use sqlx::{Postgres, QueryBuilder};

use crate::api::{
    auth::UserWithSettings,
    errors::Error,
    persons::{
        create_person_with_record, get_record_from_coord, Person, PlaceRecord, UserResponse,
    },
    MyState,
};

use super::{
    filter_persons::filter_person_query, PaginationFilterQuery, SimplePerson, UserQueryResult,
    UserView,
};

pub async fn retrieve(
    Extension(user): Extension<UserWithSettings>,
    Query(url_query): Query<PaginationFilterQuery>,
    State(state): State<MyState>,
) -> Result<(StatusCode, Json<UserQueryResult>), Error> {
    let view = match url_query.detailed.unwrap_or(false) {
        true => UserView::Detailed,
        false => UserView::Simple,
    };
    let geocoder = ReverseGeocoder::new();
    let mut sql_query: QueryBuilder<Postgres> = QueryBuilder::new(match view {
        UserView::Simple => "SELECT id, first_name, last_name, birthday, coordinate_with_search",
        UserView::Detailed => "SELECT *",
    });
    sql_query
        .push(" FROM persons WHERE user_id = ")
        .push_bind(user.user.id);

    sql_query = filter_person_query(sql_query, url_query.filter);
    sql_query
        .push(" ORDER BY id") // TODO: manual sorting
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
                .map_err(Error::DBError)?;
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
                .map_err(Error::DBError)?;
            let persons_with_coords = create_person_with_record(persons, &geocoder);
            Ok((
                StatusCode::OK,
                Json(UserQueryResult::Detailed(persons_with_coords)),
            ))
        }
    }
}

pub async fn get_single_person(
    Extension(user): Extension<UserWithSettings>,
    State(state): State<MyState>,
    Path(person_id): Path<i32>,
) -> Result<impl IntoResponse, Error> {
    let person: Person = sqlx::query_as(r#"SELECT * FROM persons WHERE user_id = $1 AND id = $2"#)
        .bind(user.user.id)
        .bind(person_id)
        .fetch_one(&state.pool)
        .await
        .map_err(Error::DBError)?;

    let geocoder = ReverseGeocoder::new();
    let record = get_record_from_coord(&geocoder, &person.coordinate_with_search.clone());
    Ok(Json(UserResponse {
        person: person.clone(),
        record: PlaceRecord::from_coord_and_record(&person.coordinate_with_search, record),
    }))
}

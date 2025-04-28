use sqlx::{Postgres, QueryBuilder};

use super::Filtering;

pub fn filter_person_query(
    mut sql_query: QueryBuilder<'static, Postgres>,
    filter: Filtering,
) -> QueryBuilder<'static, Postgres> {
    if let Some(global_search) = filter.global_search {
        sql_query
            .push(" AND searchable @@ to_tsquery(")
            .push_bind(global_search)
            .push(")");
    }
    if let Some(Ok(known_from_source_id)) = filter.known_from_search.map(|s| s.parse::<i32>()) {
        println!("{}", known_from_source_id);
        sql_query
            .push(" AND known_from_source_id = ")
            .push_bind(known_from_source_id);
    }
    sql_query
}

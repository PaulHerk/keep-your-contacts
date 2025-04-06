use api::{add_person, get_metadata, retrieve, MyState};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
pub mod api;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/persons", post(add_person).get(get_metadata::get_metadata))
        .route("/persons/{id}", get(retrieve))
        .with_state(state);

    Ok(router.into())
}

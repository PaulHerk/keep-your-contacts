use api::{add_person, delete_person, retrieve, MyState};
use axum::{
    routing::{delete, post},
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
    sqlx::query(
"CREATE TABLE IF NOT EXISTS persons (id serial PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT, city TEXT NOT NULL, note TEXT)")
        .execute(&pool).await.expect("Failed to create table");
    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    let state = MyState { pool };
    let router = Router::new()
        .route("/persons", post(add_person).get(retrieve))
        .route("/persons/delete-person/{id}", delete(delete_person))
        .with_state(state);

    Ok(router.into())
}

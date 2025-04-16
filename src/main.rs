use api::{add_person::add_person, retrieve::retrieve, MyState};
use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::{delete, get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
pub mod api;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    //     sqlx::query(
    // "CREATE TABLE IF NOT EXISTS persons (id serial PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT, city TEXT NOT NULL, job TEXT, note TEXT)")
    //         .execute(&pool).await.expect("Failed to create table");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // for cross-origin requests
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let state = MyState { pool };
    let router = Router::new()
        .route("/persons", get(retrieve).post(add_person))
        // .route("/persons/delete-person/{id}", delete(delete_person))
        .with_state(state)
        .layer(cors);

    Ok(router.into())
}

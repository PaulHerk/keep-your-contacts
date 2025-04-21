use api::{
    auth::{self, create_auth_router, jwt_auth_middleware::auth},
    get_persons::retrieve,
    post_person::add_person,
    MyState,
};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware,
    routing::{delete, get, post},
    Router,
};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
pub mod api;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
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
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let state = MyState { pool, secrets };
    let router = Router::new()
        .route("/persons", get(retrieve).post(add_person))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .merge(create_auth_router(state.clone()))
        // .route("/persons/delete-person/{id}", delete(delete_person))
        .with_state(state)
        .layer(cors);

    Ok(router.into())
}

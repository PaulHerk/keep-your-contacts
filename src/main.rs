use api::{
    auth::{create_auth_router, jwt_auth_middleware::auth},
    known_from_sources::create_known_from_sources_router,
    persons::create_persons_router,
    MyState, Secrets,
};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware, Router,
};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
pub mod api;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    //     sqlx::query(
    // "CREATE TABLE IF NOT EXISTS persons (id serial PRIMARY KEY, first_name TEXT NOT NULL, last_name TEXT, city TEXT NOT NULL, job TEXT, note TEXT)")
    //         .execute(&pool).await.expect("Failed to create table");
    let secrets = Secrets::from_secret_store(secret_store);
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // for cross-origin requests
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true);

    let state = MyState { pool, secrets };
    let router = Router::new()
        .merge(create_persons_router())
        .merge(create_known_from_sources_router())
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .merge(create_auth_router(state.clone()))
        .with_state(state)
        .layer(cors);

    Ok(router.into())
}

use api::{
    auth::{create_auth_router, jwt_auth_middleware::auth},
    known_from_sources::create_known_from_sources_router,
    persons::create_persons_router,
    MyState, Secrets,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    middleware, Router,
};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
pub mod api;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "RUST_LOG=trace,tower_http=trace,axum::rejection=trace", // .into(),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    let secrets = Secrets::from_secret_store(secret_store);

    sqlx::query("DROP TABLE _sqlx_migrations;")
        .execute(&pool)
        .await
        .expect("Failed to delete _sqlx_migrations table.");
    sqlx::query("ALTER TABLE KnownFromSources ADD COLUMN IF NOT EXISTS location_search TEXT NULL")
        .execute(&pool)
        .await
        .expect("Failed to alter table.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // for cross-origin requests
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_origin([
            "http://localhost:5173".parse().unwrap(),
            "https://keep-your-contacts.vercel.app".parse().unwrap(),
        ])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true);

    let state = MyState { pool, secrets };
    let router = Router::new()
        .merge(create_persons_router())
        .merge(create_known_from_sources_router())
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
        .merge(create_auth_router(state.clone()))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    Ok(router.into())
}

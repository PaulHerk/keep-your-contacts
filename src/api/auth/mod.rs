use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use jwt_auth_middleware::auth;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

use self::handler::*;
use super::MyState;

mod handler;
pub mod jwt_auth_middleware;
mod responses;

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}
#[derive(Deserialize)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUserSchema {
    pub name: String,
    pub password: String,
}

pub fn create_auth_router(state: MyState) -> Router<MyState> {
    Router::new()
        .route("/auth/register", post(register_user_handler))
        .route("/auth/login", post(login_user_handler))
        .route(
            "/auth/logout",
            get(logout_handler).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
        .route(
            "/auth/me",
            get(get_me_handler).route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
}

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
use optfield::optfield;

mod handler;
pub mod jwt_auth_middleware;
mod responses;

/// Database user model containing all fields including sensitive data.
/// Only used within internal systems.
#[derive(Deserialize, Serialize, FromRow, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    #[sqlx(json)]
    pub settings: DBSettings,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// This struct contains all settings. If they aren't in the DB it will default them.
#[optfield(pub DBSettings, attrs = add(derive(FromRow)), from, merge_fn = pub(self), doc = "This struct contains all fields as in FullSettings, but wrapped in Option<>.
    This is useful when adding new settings (server-side), so parsing won't fail if the attributes aren't present in the DB.")]
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullSettings {
    pub per_page: i32,
}

impl Default for FullSettings {
    fn default() -> Self {
        Self { per_page: 10 }
    }
}

/// JWT claim structure for token payload.
/// Contains user ID and timestamp information.
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

/// Router factory for all authentication endpoints.
/// Applies JWT middleware to protected routes.
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
            get(get_me_handler)
                .put(update_settings)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth)),
        )
}

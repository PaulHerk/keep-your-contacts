use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{
    extract::CookieJar,
    headers::{self, authorization::Bearer},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::{auth::UserSettings, errors::Error, MyState};

use super::{TokenClaims, User, UserWithSettings};

/// Middleware that validates JWT tokens from either cookies or Authorization header.
/// Attaches validated User to request extensions for downstream handlers.
pub async fn auth(
    cookie_jar: CookieJar,
    state: State<MyState>,
    authorization_header: Option<TypedHeader<headers::Authorization<Bearer>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    // Extract token from cookie or Bearer header
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| authorization_header.map(|auth_header| auth_header.token().to_string()));

    // Return error if token is not found
    let token = token.ok_or_else(|| Error::NotLoggedIn)?;

    // Decode and validate the JWT token
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.secrets.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidToken)?
    .claims;

    // Parse user ID from token claims
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| Error::InvalidToken)?;

    // Fetch user from database using user ID
    let user = sqlx::query_as!(User, "SELECT * FROM Users WHERE id = $1", user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(Error::DBError)?;

    // Return error if user does not exist
    let user = user.ok_or_else(|| Error::InvalidUserName)?;

    // Fetch user settings from db
    let user_settings = sqlx::query_as!(
        UserSettings,
        "SELECT * FROM UserSettings WHERE id = $1",
        user.settings_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(Error::DBError)?;

    // Attach user to request extensions, run next middleware
    req.extensions_mut().insert(UserWithSettings {
        user,
        settings: user_settings,
    });
    Ok(next.run(req).await)
}

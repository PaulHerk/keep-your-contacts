use axum::{
    body::Body,
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::{errors::Error, MyState};

use super::{TokenClaims, User};

/// Middleware that validates JWT tokens from either cookies or Authorization header.
/// Attaches validated User to request extensions for downstream handlers.
pub async fn auth(
    cookie_jar: CookieJar,
    state: State<MyState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    // Extract token from cookie or Bearer header
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| match auth_value.starts_with("Bearer ") {
                    true => Some(auth_value[7..].to_owned()),
                    false => None,
                })
        });

    // Return error if token is not found
    let token = token.ok_or_else(|| Error::NotLoggedIn)?;

    // Decode and validate the JWT token
    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.secrets.get("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::InvalidToken)?
    .claims;

    // Parse user ID from token claims
    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| Error::InvalidToken)?;

    // Fetch user from database using user ID
    let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| Error::DBError(e))?;

    // Return error if user does not exist
    let user = user.ok_or_else(|| Error::InvalidUserName)?;

    // Attach user to request extensions, run next middleware
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

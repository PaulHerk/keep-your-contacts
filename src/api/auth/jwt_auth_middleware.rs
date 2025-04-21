use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::api::MyState;

use super::{TokenClaims, User};

pub async fn auth(
    cookie_jar: CookieJar,
    state: State<MyState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, impl IntoResponse)> {
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

    let token = token.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "You aren't logged in.".to_string(),
        )
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(state.secrets.get("JWT_SECRET").unwrap().as_ref()),
        &Validation::default(),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e).to_string(),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "This user does not exist.".to_string(),
        )
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

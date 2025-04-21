use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::api::{auth::responses::FilteredUser, MyState};

use super::{LoginUserSchema, RegisterUserSchema, TokenClaims, User};

/// Creates a new user account with password hashing using Argon2.
/// Prevents duplicate names by checking existence first.
pub async fn register_user_handler(
    State(data): State<MyState>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM Users WHERE name = $1)")
            .bind(body.name.to_owned().to_ascii_lowercase())
            .fetch_one(&data.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?;

    if let Some(exists) = user_exists {
        if exists {
            return Err((
                StatusCode::CONFLICT,
                "User with that name already exists".to_string(),
            ));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while hashing password: {}", e),
            )
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO Users (name,email,password) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(body.name.to_string())
    .bind(body.email.to_string().to_ascii_lowercase())
    .bind(hashed_password)
    .fetch_one(&data.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")))?;

    Ok(StatusCode::CREATED)
}

/// Authenticates user and issues a JWT token valid for 24 hours.
/// Returns token via both JSON response and httpOnly cookie.
pub async fn login_user_handler(
    State(data): State<MyState>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM Users WHERE name = $1")
        .bind(body.name.to_ascii_lowercase())
        .fetch_optional(&data.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("DB error: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Invalid name".to_string()))?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err((StatusCode::BAD_REQUEST, "Invalid password".to_string()));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.secrets.get("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(24))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

/// Invalidates the authentication by expiring the token cookie.
pub async fn logout_handler() -> Result<impl IntoResponse, StatusCode> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

/// Returns the authenticated user's data, excluding sensitive fields.
pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": FilteredUser::from_user(user)
        })
    });

    Ok(Json(json_response))
}

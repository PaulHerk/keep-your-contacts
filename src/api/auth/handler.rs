use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::api::{auth::responses::FilteredUser, errors::Error, MyState};

use super::{
    FullSettings, LoginUserSchema, RegisterUserSchema, TokenClaims, User, UserSettings,
    UserWithSettings,
};

/// Creates a new user account with password hashing using Argon2.
/// Prevents duplicate names by checking existence first.
pub async fn register_user_handler(
    State(data): State<MyState>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, Error> {
    let user_exists: Option<bool> = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM Users WHERE name = $1)",
        body.name.to_owned().to_ascii_lowercase()
    )
    .fetch_one(&data.pool)
    .await
    .map_err(Error::DBError)?;

    if let Some(exists) = user_exists {
        if exists {
            return Err(Error::UserAlreadyExists);
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_ref(), &salt)
        .map_err(Error::HashingError)
        .map(|hash| hash.to_string())?;

    let default_settings = UserSettings::from(FullSettings::default());
    let settings_id = sqlx::query_as!(
        SettingsId,
        "INSERT INTO UserSettings (per_page) VALUES ($1) RETURNING id",
        default_settings.per_page,
    )
    .fetch_one(&data.pool)
    .await
    .map_err(Error::DBError)?;

    sqlx::query!(
        "INSERT INTO Users (name, email, settings_id, password) VALUES ($1, $2, $3, $4)",
        body.name.to_string(),
        body.email.to_string().to_ascii_lowercase(),
        settings_id.id,
        hashed_password,
    )
    .execute(&data.pool)
    .await
    .map_err(Error::DBError)?;

    Ok((StatusCode::CREATED, Redirect::to("/auth/login")))
}
struct SettingsId {
    id: i32,
}

/// Authenticates user and issues a JWT token valid for 24 hours.
/// Returns token via both JSON response and httpOnly cookie.
pub async fn login_user_handler(
    State(data): State<MyState>,
    jar: CookieJar,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM Users WHERE name = $1",
        body.name.to_ascii_lowercase()
    )
    .fetch_optional(&data.pool)
    .await
    .map_err(Error::DBError)?
    .ok_or_else(|| Error::InvalidLoginName)?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_ref(), &parsed_hash)
            .is_ok_and(|_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(Error::InvalidPassword);
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp =
        (now + chrono::Duration::hours(data.secrets.login_expired as i64)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.secrets.jwt_secret.as_bytes()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(data.secrets.login_expired as i64))
        .same_site(SameSite::Lax)
        .http_only(true);

    Ok((
        StatusCode::ACCEPTED,
        jar.add(cookie.clone()),
        cookie.clone().to_string(), // I have to return the cookie in the body since svelte doesn't evalute set-cookie headers so I have to do this manually in the login form request. https://svelte.dev/docs/kit/load#Cookies
    ))
}

/// Invalidates the authentication by expiring the token cookie.
pub async fn logout_handler(jar: CookieJar) -> Result<impl IntoResponse, Error> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    Ok((StatusCode::OK, jar.add(cookie)))
}

/// Returns the authenticated user's data, excluding sensitive fields.
pub async fn get_me_handler(
    Extension(user): Extension<UserWithSettings>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(FilteredUser::from_user(user)))
}

/// Updates the authenticated user's settings.
pub async fn update_settings(
    State(data): State<MyState>,
    Extension(mut user): Extension<UserWithSettings>,
    Json(new_settings): Json<UserSettings>,
) -> Result<impl IntoResponse, Error> {
    sqlx::query!(
        "UPDATE UserSettings SET per_page = $1 WHERE id = $2",
        new_settings.per_page,
        user.settings.id
    )
    .execute(&data.pool)
    .await
    .map_err(Error::DBError)?;

    user.settings = new_settings;
    Ok(StatusCode::OK)
}

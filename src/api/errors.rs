use argon2::password_hash;
use axum::{http::StatusCode, response::IntoResponse};
use geocoding::GeocodingError;

pub enum Error {
    DBError(sqlx::Error),
    PersonNotFound,
    KnownFromSourceNotFound,
    CityNotFound(GeocodingError),
    HashingError(password_hash::Error),
    UserAlreadyExists,
    InvalidLoginName,
    InvalidPassword,
    NotLoggedIn,
    InvalidToken,
    InvalidUserName,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::DBError(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}")),
            Error::HashingError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while hashing password: {e}"),
            ),
            Error::PersonNotFound => (
                StatusCode::NOT_FOUND,
                "This person was not found.".to_string(),
            ),
            Error::KnownFromSourceNotFound => (
                StatusCode::NOT_FOUND,
                "This known from source was not found.".to_string(),
            ),
            Error::CityNotFound(e) => (
                StatusCode::NOT_FOUND,
                format!("This city was not found. Error: {e}"),
            ),
            Error::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "A user with that name already exists.".to_string(),
            ),
            Error::InvalidLoginName => (StatusCode::BAD_REQUEST, "Invalid username".to_string()),
            Error::InvalidPassword => (StatusCode::BAD_REQUEST, "Invalid password".to_string()),
            Error::NotLoggedIn => (
                StatusCode::UNAUTHORIZED,
                "You aren't logged in.".to_string(),
            ),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            Error::InvalidUserName => (
                StatusCode::UNAUTHORIZED,
                "This user does not exist.".to_string(),
            ),
        }
        .into_response()
    }
}

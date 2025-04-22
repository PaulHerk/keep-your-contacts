use argon2::password_hash;
use axum::{http::StatusCode, response::IntoResponse};

pub enum Error {
    DBError(sqlx::Error),
    PersonNotFound,
    KnownFromSourceNotFound,
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
            Error::PersonNotFound => (StatusCode::NOT_FOUND, format!("This person was not found.")),
            Error::KnownFromSourceNotFound => (
                StatusCode::NOT_FOUND,
                format!("This known from source was not found."),
            ),
            Error::UserAlreadyExists => (
                StatusCode::CONFLICT,
                format!("A user with that name already exists."),
            ),
            Error::InvalidLoginName => (StatusCode::BAD_REQUEST, format!("Invalid username")),
            Error::InvalidPassword => (StatusCode::BAD_REQUEST, format!("Invalid password")),
            Error::NotLoggedIn => (StatusCode::UNAUTHORIZED, format!("You aren't logged in.")),
            Error::InvalidToken => (StatusCode::UNAUTHORIZED, format!("Invalid token")),
            Error::InvalidUserName => (
                StatusCode::UNAUTHORIZED,
                format!("This user does not exist."),
            ),
        }
        .into_response()
    }
}

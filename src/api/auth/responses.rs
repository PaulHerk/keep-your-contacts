use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};

use super::User;

/// Sanitized user representation that excludes sensitive data like passwords.
/// Used for all user-facing responses.
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

/// Converts internal User model to public FilteredUser representation.
/// Ensures sensitive data never leaves the system.
impl FilteredUser {
    pub fn from_user(user: User) -> Self {
        FilteredUser {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            createdAt: user.created_at.unwrap(),
            updatedAt: user.updated_at.unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

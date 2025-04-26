use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};

use super::User;

/// Sanitized user representation that excludes sensitive data like passwords.
/// Used for all user-facing responses.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Converts internal User model to public FilteredUser representation.
/// Ensures sensitive data never leaves the system.
impl FilteredUser {
    pub fn from_user(user: User) -> Self {
        FilteredUser {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Serialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

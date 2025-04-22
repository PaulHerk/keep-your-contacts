use shuttle_runtime::SecretStore;
use sqlx::PgPool;

pub mod auth;
pub mod errors;
pub mod known_from_sources;
pub mod persons;

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
    pub secrets: Secrets,
}

#[derive(Clone)]
pub struct Secrets {
    jwt_secret: String,
    login_expired: u8, // in hours
}

impl Secrets {
    pub fn from_secret_store(secret_store: SecretStore) -> Self {
        let jwt_secret = secret_store.get("JWT_SECRET").unwrap();
        let login_expired = secret_store.get("LOGIN_EXPIRED").unwrap().parse().unwrap();
        Self {
            jwt_secret,
            login_expired,
        }
    }
}

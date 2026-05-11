use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{storage::entities::user::UserEntity, utils::time::now};

pub mod jwt;

#[derive(Debug, Error)]
pub enum AuthTokenError {
    #[error("{0:?}")]
    Anyhow(#[from] anyhow::Error),
    #[error("jwt error: {0:?}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("validation error: {0}")]
    Validation(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub user_id: String,
    pub user_role: String,
    pub exp: usize,
}

impl From<UserEntity> for Claims {
    fn from(value: UserEntity) -> Self {
        Self {
            user_id: value.id.to_string(),
            user_role: value.role.into(),
            exp: Utc::now().checked_add_signed(Duration::days(10)).expect("valid timestamp").timestamp() as usize,
        }
    }
}

pub trait AuthTokenTrait: Clone + Send + Sync + 'static {
    fn generate(&self, claims: Claims) -> Result<String, AuthTokenError>;
    fn validate(&self, token: &str) -> Result<Claims, AuthTokenError>;
}

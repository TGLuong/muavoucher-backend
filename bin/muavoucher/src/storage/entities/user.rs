use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    #[default]
    User,
    Admin,
    CookieManager,
    ProductManager,
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "user" => Self::User,
            "admin" => Self::Admin,
            "cookie_manager" => Self::CookieManager,
            "product_manager" => Self::ProductManager,
            _ => Self::User,
        }
    }
}

impl Into<String> for UserRole {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::CookieManager => write!(f, "cookie_manager"),
            UserRole::ProductManager => write!(f, "product_manager"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub user_name: Option<String>,
    pub password: String,
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

impl From<UserEntity> for GetUserResponse {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            user_name: value.user_name,
            full_name: value.full_name,
            avatar: value.avatar,
            phone: value.phone,
            email: value.email,
            role: Some(value.role.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub password: String,
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePasswordRequest {
    pub old: String,
    pub new: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserRequest {
    pub user_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub token: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserOtpType {
    Phone,
    Email,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserFilter {
    pub id: Option<Uuid>,
    pub user_name: Option<String>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl UserFilter {
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user_name = Some(user);
        self
    }

    pub fn with_phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum UserOrder {
    #[default]
    CreatedAt,
}

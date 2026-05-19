use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::Utc;
use sea_orm::{ActiveValue::Set, prelude::*};
use uuid::Uuid;

use crate::storage::{
    DatabaseError,
    entities::user::{CreateUserRequest, UpdateUserRequest, UserEntity, UserRole},
};

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    password: String,
    full_name: Option<String>,
    avatar: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    role: String,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    balance: Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    coin: Decimal,
    #[sea_orm(column_type = "Decimal(Some((15, 2)))")]
    affiliate_coin: Decimal,
    created_at: chrono::DateTime<Utc>,
    updated_at: Option<chrono::DateTime<Utc>>,
    deleted_at: Option<chrono::DateTime<Utc>>,
}

impl Into<UserEntity> for Model {
    fn into(self) -> UserEntity {
        UserEntity {
            id: self.id,
            password: self.password,
            full_name: self.full_name,
            avatar: self.avatar,
            phone: self.phone,
            email: self.email,
            role: self.role.into(),
        }
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
    #[sea_orm(has_many = "super::transaction_history::Entity")]
    TransactionHistory,
}

impl Related<super::order::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Order.def()
    }
}

impl Related<super::transaction_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TransactionHistory.def()
    }
}

impl ActiveModel {
    pub fn patch(&mut self, request: UpdateUserRequest) -> bool {
        let mut is_change = false;
        if let Some(full_name) = request.full_name {
            self.full_name = Set(Some(full_name));
            is_change = true;
        }
        if let Some(avatar) = request.avatar {
            self.avatar = Set(Some(avatar));
            is_change = true;
        }
        if let Some(phone) = request.phone {
            self.phone = Set(Some(phone));
            is_change = true;
        }
        if let Some(email) = request.email {
            self.email = Set(Some(email));
            is_change = true;
        }
        is_change
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl TryFrom<CreateUserRequest> for ActiveModel {
    type Error = DatabaseError;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        let id = Uuid::now_v7();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(value.password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("hash password error: {e:?}"))?
            .to_string();
        Ok(Self {
            id: Set(id),
            password: Set(password),
            full_name: Set(value.full_name),
            avatar: Set(value.avatar),
            phone: Set(value.phone),
            email: Set(value.email),
            role: Set(UserRole::default().into()),
            created_at: Set(Utc::now().into()),
            ..Self::new()
        })
    }
}

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use sea_orm_migration::prelude::*;
use uuid::Uuid;

use crate::storage::entities::user::UserRole;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user = std::env::var("ADMIN_USER_NAME").unwrap_or("admin".to_string());
        let pass = std::env::var("ADMIN_PASSWORD").unwrap_or("admin@123".to_string());
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password = argon2
            .hash_password(pass.as_bytes(), &salt)
            .map_err(|e| DbErr::Custom(format!("hash password error: {e:?}")))?
            .to_string();
        let stmt = Query::insert()
            .into_table(User::Table)
            .columns([
                User::Id,
                User::UserName,
                User::Password,
                User::Role,
                User::Balance,
                User::Coin,
                User::AffiliateCoin,
            ])
            .values_panic(vec![
                Expr::val(Uuid::now_v7()).into(),
                Expr::val(user).into(),
                Expr::val(password).into(),
                Expr::val(UserRole::Admin.to_string()).into(),
                Expr::val(0).into(),
                Expr::val(0).into(),
                Expr::val(0).into(),
            ])
            .to_owned();
        manager.exec_stmt(stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let user = std::env::var("ADMIN_USER_NAME").unwrap_or("admin".to_string());
        let stmt = Query::delete()
            .from_table(User::Table)
            .and_where(Expr::col(User::UserName).eq(user))
            .to_owned();
        manager.exec_stmt(stmt).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    Password,
    Role,
    Balance,
    Coin,
    AffiliateCoin,
}

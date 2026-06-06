use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(User::UserName).string().null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::FullName).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::Phone).string().unique_key().null())
                    .col(ColumnDef::new(User::Email).string().unique_key().null())
                    .col(ColumnDef::new(User::Role).string().not_null())
                    .col(ColumnDef::new(User::Balance).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(User::Coin).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(User::AffiliateCoin).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TransactionHistory::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TransactionHistory::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(TransactionHistory::UserId).uuid().not_null())
                    .col(ColumnDef::new(TransactionHistory::AccountNumber).string().null())
                    .col(ColumnDef::new(TransactionHistory::Type).string().not_null())
                    .col(ColumnDef::new(TransactionHistory::Message).string().null())
                    .col(ColumnDef::new(TransactionHistory::Transaction).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(TransactionHistory::Balance).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(TransactionHistory::ReferenceCode).string().null())
                    .col(
                        ColumnDef::new(TransactionHistory::TransactionDate)
                            .timestamp_with_time_zone()
                            .default("now()"),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Order::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Order::UserId).uuid().null())
                    .col(ColumnDef::new(Order::OrderDate).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(Order::ProductName).string().not_null())
                    .col(ColumnDef::new(Order::OrderCode).string().unique_key().not_null())
                    .col(ColumnDef::new(Order::OrderValue).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Order::Commission).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(Order::Status).string().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager.drop_table(Table::drop().table(User::Table).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(TransactionHistory::Table).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(Order::Table).to_owned()).await;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    UserName,
    Password,
    FullName,
    Avatar,
    Phone,
    Email,
    Role,
    Balance,
    Coin,
    AffiliateCoin,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(DeriveIden)]
enum TransactionHistory {
    Table,
    Id,
    UserId,
    AccountNumber,
    Type,
    Message,
    Transaction,
    Balance,
    ReferenceCode,
    TransactionDate,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    OrderDate,
    ProductName,
    OrderCode,
    OrderValue,
    Commission,
    Status,
}

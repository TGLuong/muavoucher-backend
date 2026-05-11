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
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::FullName).string().null())
                    .col(ColumnDef::new(User::Avatar).string().null())
                    .col(ColumnDef::new(User::Phone).string().unique_key().null())
                    .col(ColumnDef::new(User::Email).string().unique_key().null())
                    .col(ColumnDef::new(User::Role).string().not_null())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().null())
                    .col(ColumnDef::new(User::DeletedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager.drop_table(Table::drop().table(User::Table).to_owned()).await;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Password,
    FullName,
    Avatar,
    Phone,
    Email,
    Role,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

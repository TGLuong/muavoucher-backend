use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Area::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Area::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Area::Name).string().null())
                    .col(ColumnDef::new(Area::From).decimal_len(20, 2).not_null())
                    .col(ColumnDef::new(Area::To).decimal_len(20, 2).not_null())
                    .col(ColumnDef::new(Area::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(Area::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Cookie::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Cookie::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Cookie::Area).uuid().null())
                    .col(ColumnDef::new(Cookie::Name).string().null())
                    .col(ColumnDef::new(Cookie::Cookie).string().not_null())
                    .col(ColumnDef::new(Cookie::Priority).integer().not_null())
                    .col(ColumnDef::new(Cookie::Status).boolean().default(SimpleExpr::Value(true.into())))
                    .col(ColumnDef::new(Cookie::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(Cookie::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager.drop_table(Table::drop().table(Area::Table).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(Cookie::Table).to_owned()).await;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Area {
    Table,
    Id,
    Name,
    From,
    To,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Cookie {
    Table,
    Id,
    Area,
    Name,
    Cookie,
    Priority,
    Status,
    CreatedAt,
    UpdatedAt,
}

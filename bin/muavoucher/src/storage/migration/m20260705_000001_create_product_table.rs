use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Shop::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Shop::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Shop::Name).string().not_null())
                    .col(ColumnDef::new(Shop::Description).string().null())
                    .col(ColumnDef::new(Shop::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(Shop::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Product::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Product::ShopId).uuid().not_null())
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(ColumnDef::new(Product::Image).string().not_null())
                    .col(ColumnDef::new(Product::OriginLink).string().not_null())
                    .col(ColumnDef::new(Product::AffiliateLink).string().not_null())
                    .col(ColumnDef::new(Product::Price).decimal_len(20, 2).not_null())
                    .col(ColumnDef::new(Product::Commission).decimal_len(20, 2).not_null())
                    .col(ColumnDef::new(Product::CreatedAt).timestamp_with_time_zone().default("now()"))
                    .col(ColumnDef::new(Product::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager.drop_table(Table::drop().table(Shop::Table).to_owned()).await;
        let _ = manager.drop_table(Table::drop().table(Product::Table).to_owned()).await;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Shop {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    ShopId,
    Name,
    Image,
    OriginLink,
    AffiliateLink,
    Price,
    Commission,
    CreatedAt,
    UpdatedAt,
}

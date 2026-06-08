use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(Index::create().table(Area::Table).name("idx-area-from").col(Area::From).to_owned())
            .await?;
        manager
            .create_index(Index::create().table(Area::Table).name("idx-area-to").col(Area::To).to_owned())
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Cookie::Table)
                    .name("idx-cookie-priority")
                    .col((Cookie::Priority, IndexOrder::Asc))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Cookie::Table)
                    .name("idx-cookie-status")
                    .col(Cookie::Status)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().table(Area::Table).name("idx-area-from").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().table(Area::Table).name("idx-area-to").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().table(Cookie::Table).name("idx-cookie-priority").to_owned())
            .await?;
        manager
            .drop_index(Index::drop().table(Cookie::Table).name("idx-cookie-status").to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Area {
    Table,
    From,
    To,
}

#[derive(DeriveIden)]
enum Cookie {
    Table,
    Priority,
    Status,
}

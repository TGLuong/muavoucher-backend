# Migration Rules

These rules apply to `bin/muavoucher/src/storage/migration/`.

## Location And Naming

- Put all migration files in `bin/muavoucher/src/storage/migration/`.
- Name migration files using the existing format: `mYYYYMMDD_NNNNNN_<action>.rs`.
- Use a short snake_case action name that describes the schema or data change.
- Follow the ordering style already present in `mod.rs`.

## SeaORM Migration Style

- Use the existing SeaORM migration structure:
  - `use sea_orm_migration::prelude::*;`
  - `#[derive(DeriveMigrationName)]`
  - `pub struct Migration;`
  - `impl MigrationTrait for Migration`
  - `SchemaManager`
  - `DeriveIden` enums for table and column identifiers
- Prefer SeaORM migration builders such as `Table::create`, `Table::alter`, `ColumnDef`, `Index::create`, `Index::drop`, and `Query::*`.
- Avoid raw SQL. Use raw SQL only when SeaORM migration builders cannot express the change, and explain why in the response.

## Registration

- When adding a migration file, update `bin/muavoucher/src/storage/migration/mod.rs` with `pub mod <migration_name>;`.
- Add the migration to `MigratorTrait::migrations()` when it should run as part of the migration chain.
- Do not remove or reorder existing migrations unless explicitly requested.

## Safety

- Keep `up` and `down` behavior paired when practical.
- Do not silently drop data in `down` beyond reversing the migration's own changes.
- Follow existing table, column, index, and enum naming patterns.

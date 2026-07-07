use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub mod m20260510_000001_create_table;
pub mod m20260604_000001_create_area_cokie;
pub mod m20260606_000001_add_admin;
pub mod m20260606_000002_create_index_for_cookie_area;
pub mod m20260705_000001_create_product_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260510_000001_create_table::Migration),
            Box::new(m20260604_000001_create_area_cokie::Migration),
            Box::new(m20260606_000001_add_admin::Migration),
            Box::new(m20260705_000001_create_product_table::Migration),
        ]
    }
}

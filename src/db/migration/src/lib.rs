pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240122_233344_add_photo_to_product;
mod m20240123_231042_add_admins_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240122_233344_add_photo_to_product::Migration),
            Box::new(m20240123_231042_add_admins_table::Migration),
        ]
    }
}

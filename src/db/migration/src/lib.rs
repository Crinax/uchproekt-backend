pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240122_233344_add_photo_to_product;
mod m20240123_231042_add_admins_table;
mod m20240124_224933_add_service_table;
mod m20240124_225823_add_field_types;
mod m20240124_232016_add_field_table;
mod m20240124_232904_add_category_table;
mod m20240124_234641_add_order_table;
mod m20240124_235342_add_products_in_order_table;
mod m20240125_000958_add_category_product_table;
mod m20240125_001325_add_field_product_table;
mod m20240317_014139_add_file_table;
mod m20240317_031542_change_product_file_type_to_uuid_and_link_with_file_table;
mod m20240512_093640_remove_nullable_fields_for_order;
mod m20240713_170813_define_order_product_stable_relation;
mod m20240713_174747_stable_category_product_relationships;
mod m20240713_175628_stable_field_product_relationships;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240122_233344_add_photo_to_product::Migration),
            Box::new(m20240123_231042_add_admins_table::Migration),
            Box::new(m20240124_224933_add_service_table::Migration),
            Box::new(m20240124_225823_add_field_types::Migration),
            Box::new(m20240124_232016_add_field_table::Migration),
            Box::new(m20240124_232904_add_category_table::Migration),
            Box::new(m20240124_234641_add_order_table::Migration),
            Box::new(m20240124_235342_add_products_in_order_table::Migration),
            Box::new(m20240125_000958_add_category_product_table::Migration),
            Box::new(m20240125_001325_add_field_product_table::Migration),
            Box::new(m20240317_014139_add_file_table::Migration),
            Box::new(m20240317_031542_change_product_file_type_to_uuid_and_link_with_file_table::Migration),
            Box::new(m20240512_093640_remove_nullable_fields_for_order::Migration),
            Box::new(m20240713_170813_define_order_product_stable_relation::Migration),
            Box::new(m20240713_174747_stable_category_product_relationships::Migration),
            Box::new(m20240713_175628_stable_field_product_relationships::Migration),
        ]
    }
}

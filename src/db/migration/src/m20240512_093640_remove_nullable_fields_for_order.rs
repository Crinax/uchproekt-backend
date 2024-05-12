use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let query = Table::alter()
            .table(ProductsInOrder::Table)
            .drop_column(ProductsInOrder::ProductId)
            .drop_column(ProductsInOrder::OrderId)
            .add_column(
                ColumnDef::new(ProductsInOrder::ProductId)
                    .integer()
                    .not_null(),
            )
            .add_column(
                ColumnDef::new(ProductsInOrder::OrderId)
                    .integer()
                    .not_null(),
            )
            .add_foreign_key(
                TableForeignKey::new()
                    .name("fk_products_in_order_product")
                    .from_tbl(ProductsInOrder::Table)
                    .from_col(ProductsInOrder::ProductId)
                    .to_tbl(Product::Table)
                    .to_col(Product::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .add_foreign_key(
                TableForeignKey::new()
                    .name("fk_products_in_order_order")
                    .from_tbl(ProductsInOrder::Table)
                    .from_col(ProductsInOrder::OrderId)
                    .to_tbl(Order::Table)
                    .to_col(Order::Id)
                    .on_delete(ForeignKeyAction::SetNull),
            )
            .to_owned();

        manager.alter_table(query.to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProductsInOrder::Table)
                    .drop_column(ProductsInOrder::ProductId)
                    .drop_column(ProductsInOrder::OrderId)
                    .add_column(ColumnDef::new(ProductsInOrder::ProductId).integer())
                    .add_column(ColumnDef::new(ProductsInOrder::OrderId).integer())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_products_in_order_product")
                            .from_tbl(ProductsInOrder::Table)
                            .from_col(ProductsInOrder::ProductId)
                            .to_tbl(Product::Table)
                            .to_col(Product::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_products_in_order_order")
                            .from_tbl(ProductsInOrder::Table)
                            .from_col(ProductsInOrder::OrderId)
                            .to_tbl(Order::Table)
                            .to_col(Order::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ProductsInOrder {
    Table,
    ProductId,
    OrderId,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
}

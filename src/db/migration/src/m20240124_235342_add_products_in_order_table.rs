use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductsInOrder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductsInOrder::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductsInOrder::ProductId).integer())
                    .col(ColumnDef::new(ProductsInOrder::OrderId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_in_order_product")
                            .from(ProductsInOrder::Table, ProductsInOrder::ProductId)    
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_in_order_order")
                            .from(ProductsInOrder::Table, ProductsInOrder::OrderId)    
                            .to(Order::Table, Order::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductsInOrder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
#[sea_orm(iden = "products_in_order")]
enum ProductsInOrder {
    Table,
    Id,
    ProductId,
    OrderId,
}

#[derive(DeriveIden)]
enum Product {
    Table,
    Id
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id
}

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProductsInOrder::Table)
                    .add_column(
                        ColumnDef::new(ProductsInOrder::Quantity)
                            .not_null()
                            .integer()
                            .default(1),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ProductsInOrder::Table)
                    .drop_column(ProductsInOrder::Quantity)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ProductsInOrder {
    Table,
    Quantity,
}
